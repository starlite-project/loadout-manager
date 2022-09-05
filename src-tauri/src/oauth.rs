use std::{
	fmt::{Display, Formatter, Result as FmtResult},
	time::{Duration, SystemTime},
};

use futures_util::{SinkExt, StreamExt};
use oauth2::{
	basic::{
		BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
		BasicTokenType,
	},
	AccessToken, AuthorizationCode, Client as OAuth2Client, CsrfToken, ExtraTokenFields,
	PkceCodeChallenge, RefreshToken, StandardRevocableToken, StandardTokenResponse, TokenResponse,
};
use serde::{Deserialize, Serialize};
use tauri::{
	api::http::{Body, HttpRequestBuilder, ResponseType},
	Manager,
};
use tokio_tungstenite::{
	connect_async,
	tungstenite::{
		protocol::{frame::coding::CloseCode, CloseFrame},
		Message,
	},
};

use super::{plugins::Store, util::API_KEY, Result};
use crate::{util::Impossible, LoadoutClient};

#[tauri::command]
pub async fn get_authorization_code(
	app_handle: tauri::AppHandle,
	http: tauri::State<'_, LoadoutClient>,
	storage: tauri::State<'_, Store>,
) -> Result<()> {
	let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

	let (auth_url, csrf_token) = http
		.authorize_url(CsrfToken::new_random)
		.set_pkce_challenge(pkce_challenge)
		.url();

	let mut ws = connect_async("wss://localhost:3030/socket").await?.0;

	let data_to_send = serde_json::json!({
		"api_key": API_KEY.to_owned(),
		"state": csrf_token.secret().to_owned(),
	});

	ws.send(Message::Text(serde_json::to_string(&data_to_send)?))
		.await?;

	let scope = app_handle.shell_scope();

	scope.open(auth_url.as_str(), None)?;

	let mut raw_code = String::new();
	while let Some(Ok(c)) = ws.next().await {
		if c.is_ping() {
			ws.send(Message::Pong(c.into_data())).await?;
			continue;
		}

		if !c.is_text() {
			panic!("invalid message received");
		}

		raw_code.push_str(&c.to_text()?);
		let _ = ws.close(Some(CloseFrame {
			code: CloseCode::Normal,
			reason: "received code".into(),
		}))
		.await;
		break;
	}

	let (code, oauth_state) = {
		let raw_values = raw_code.split(':').collect::<Vec<_>>();
		if raw_values.len() != 2 {
			return Err(crate::Error(anyhow::anyhow!(
				"didn't receive both code and state"
			)));
		}

		(raw_values[0], raw_values[1])
	};

	log::debug!(
		"received code {} with state {} from bungie",
		code,
		oauth_state
	);

	if oauth_state != csrf_token.secret().as_str() {
		log::error!("state was invalid, something has been compromised, bailing application");

		std::process::abort();
	}

	let client = &*http;

	let token_result = http
		.exchange_code(AuthorizationCode::new(code.to_owned()))
		.set_pkce_verifier(pkce_verifier)
		.request_async(move |req| make_request(client, req))
		.await?;

	let d2_token = D2Token::try_from(token_result)?;

	storage
		.insert("auth_data".to_owned(), serde_json::to_value(&d2_token)?)
		.await;

	Ok(())
}

#[tauri::command]
pub async fn delete_token(storage: tauri::State<'_, Store>) -> Result<bool, Impossible> {
	let flag = storage.remove("auth_data").await.is_some();
	Ok(flag)
}

#[tauri::command]
pub async fn refresh_token(
	http: tauri::State<'_, LoadoutClient>,
	storage: tauri::State<'_, Store>,
) -> Result<()> {
	let old_auth_data = storage.get("auth_data").await.expect("no auth data found");

	let json = serde_json::from_value::<D2Token>(old_auth_data).expect("invalid JSON");

	let refresh_token = RefreshToken::new(json.refresh_token);

	let client = &*http;

	let new_auth_data = http
		.exchange_refresh_token(&refresh_token)
		.request_async(|req| make_request(client, req))
		.await?;

	let new_token = D2Token::try_from(new_auth_data)?;

	storage
		.insert("auth_data".to_owned(), serde_json::to_value(&new_token)?)
		.await;

	Ok(())
}

#[tauri::command]
pub async fn is_token_valid(
	storage: tauri::State<'_, Store>,
) -> Result<bool, crate::util::Impossible> {
	let auth_data_nullable = storage.get("auth_data").await;

	if let Some(auth) = auth_data_nullable {
		let now = SystemTime::now();
		let expiration = match serde_json::from_value::<D2Token>(auth) {
			Ok(v) => v.expires_in,
			Err(_) => return Ok(false),
		};

		Ok(expiration > now)
	} else {
		Ok(false)
	}
}

#[tauri::command]
pub async fn is_token_refreshable(
	storage: tauri::State<'_, Store>,
) -> Result<bool, crate::util::Impossible> {
	let auth_data_nullable = storage.get("auth_data").await;

	if let Some(auth) = auth_data_nullable {
		let now = SystemTime::now();
		let refresh_expires_in = match serde_json::from_value::<D2Token>(auth) {
			Ok(v) => v.refresh_expires_in,
			Err(_) => return Ok(false),
		};

		Ok(refresh_expires_in > now)
	} else {
		Ok(false)
	}
}

// Doing this to use the same http client I use everywhere else, for consistency.
async fn make_request(
	client: &LoadoutClient,
	req: oauth2::HttpRequest,
) -> Result<oauth2::HttpResponse, oauth2::reqwest::Error<tauri::api::Error>> {
	let builder = HttpRequestBuilder::new(req.method.to_string(), &req.url)
		.map_err(oauth2::reqwest::Error::Reqwest)?
		.headers(req.headers.clone())
		.body(Body::Bytes(req.body.clone()))
		.response_type(ResponseType::Binary);

	let t_response = client
		.send(builder)
		.await
		.map_err(oauth2::reqwest::Error::Reqwest)?;

	let status = t_response.status();
	let headers = t_response.headers().clone();

	let body = t_response
		.bytes()
		.await
		.map_err(oauth2::reqwest::Error::Reqwest)?;

	Ok(oauth2::HttpResponse {
		status_code: status,
		headers,
		body: body.data,
	})
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct D2Token {
	pub access_token: String,
	pub expires_in: SystemTime,
	pub refresh_token: String,
	pub refresh_expires_in: SystemTime,
	pub membership_id: i64,
	received: SystemTime,
}

impl Default for D2Token {
	fn default() -> Self {
		let now = SystemTime::UNIX_EPOCH;
		Self {
			access_token: String::new(),
			expires_in: now,
			refresh_token: String::new(),
			refresh_expires_in: now,
			membership_id: 0,
			received: now,
		}
	}
}

impl TryFrom<D2Token> for StandardTokenResponse<D2ExtraFields, BasicTokenType> {
	type Error = ConversionError;

	fn try_from(old: D2Token) -> Result<Self, Self::Error> {
		let time_since_expires = old
			.expires_in
			.duration_since(old.received)
			.map_err(|_| ConversionError)?;
		let time_since_refresh_expires = old
			.refresh_expires_in
			.duration_since(old.received)
			.map_err(|_| ConversionError)?;
		let mut new = Self::new(
			AccessToken::new(old.access_token),
			BasicTokenType::Bearer,
			D2ExtraFields {
				refresh_expires_in: Some(time_since_refresh_expires.as_secs()),
				membership_id: old.membership_id,
			},
		);

		new.set_refresh_token(Some(RefreshToken::new(old.refresh_token)));

		new.set_expires_in(Some(&time_since_expires));

		Ok(new)
	}
}

impl TryFrom<StandardTokenResponse<D2ExtraFields, BasicTokenType>> for D2Token {
	type Error = ConversionError;

	fn try_from(
		value: StandardTokenResponse<D2ExtraFields, BasicTokenType>,
	) -> Result<Self, Self::Error> {
		let now = SystemTime::now();

		let access_token = value.access_token().secret().clone();

		let expires_in = now + value.expires_in().ok_or(ConversionError)?;

		let refresh_token = value
			.refresh_token()
			.map(|x| x.secret().clone())
			.ok_or(ConversionError)?;

		let refresh_expires_in = now
			+ value
				.extra_fields()
				.refresh_expires_in
				.map(Duration::from_secs)
				.ok_or(ConversionError)?;

		let membership_id = value.extra_fields().membership_id.clone();

		Ok(D2Token {
			access_token,
			expires_in,
			refresh_token,
			refresh_expires_in,
			membership_id,
			received: now,
		})
	}
}

#[derive(Debug)]
pub struct ConversionError;

impl Display for ConversionError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("failed to convert from standard response (this shouldn't happen and represents a bug in the Bungie API)")
	}
}

impl std::error::Error for ConversionError {}

pub type D2OAuthClient = OAuth2Client<
	BasicErrorResponse,
	StandardTokenResponse<D2ExtraFields, BasicTokenType>,
	BasicTokenType,
	BasicTokenIntrospectionResponse,
	StandardRevocableToken,
	BasicRevocationErrorResponse,
>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D2ExtraFields {
	#[serde(skip_serializing_if = "Option::is_none")]
	refresh_expires_in: Option<u64>,
	#[serde(deserialize_with = "crate::util::deserialize_string_as")]
	membership_id: i64,
}

impl ExtraTokenFields for D2ExtraFields {}
