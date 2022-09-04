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
	AuthUrl, AuthorizationCode, Client as OAuth2Client, ClientId, ClientSecret, CsrfToken,
	ExtraTokenFields, PkceCodeChallenge, StandardRevocableToken, StandardTokenResponse,
	TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use tauri::{
	api::http::{Body, Client, HttpRequestBuilder, ResponseType},
	Manager,
};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use super::{
	fetch::{API_KEY, CLIENT_ID},
	Result,
};
use crate::plugins::Store;

const CLIENT_SECRET: &str = env!("CLIENT_SECRET");

#[tauri::command]
pub async fn get_authorization_code(
	app_handle: tauri::AppHandle,
	state: tauri::State<'_, Client>,
	storage: tauri::State<'_, Store>,
) -> Result<()> {
	let client = D2OAuthClient::new(
		ClientId::new(CLIENT_ID.to_owned()),
		Some(ClientSecret::new(CLIENT_SECRET.to_owned())),
		AuthUrl::new("https://bungie.net/en/oauth/authorize/".to_owned())?,
		Some(TokenUrl::new(
			"https://www.bungie.net/Platform/App/OAuth/Token/".to_owned(),
		)?),
	);

	let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

	let (auth_url, csrf_token) = client
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

	log::debug!("received code {} with state {} from bungie", code, oauth_state);

	if oauth_state != csrf_token.secret().as_str() {
		log::error!("state was invalid, something has been compromised, bailing application");

		std::process::abort();
	}

	let http = &*state;
	let token_result = client
		.exchange_code(AuthorizationCode::new(code.to_owned()))
		.set_pkce_verifier(pkce_verifier)
		.request_async(move |req| make_request(http, req))
		.await?;

	let d2_token = D2Token::try_from(token_result)?;

	storage.insert("auth_data".to_owned(), serde_json::to_value(&d2_token)?).await;

	Ok(())
}

// Doing this to use the same http client I use everywhere else, for consistency.
async fn make_request(
	client: &tauri::api::http::Client,
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
	pub membership_id: String,
}

impl Default for D2Token {
	fn default() -> Self {
		let now = SystemTime::now();
		Self {
			access_token: String::new(),
			expires_in: now,
			refresh_token: String::new(),
			refresh_expires_in: now,
			membership_id: String::new(),
		}
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

type D2OAuthClient = OAuth2Client<
	BasicErrorResponse,
	StandardTokenResponse<D2ExtraFields, BasicTokenType>,
	BasicTokenType,
	BasicTokenIntrospectionResponse,
	StandardRevocableToken,
	BasicRevocationErrorResponse,
>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct D2ExtraFields {
	#[serde(skip_serializing_if = "Option::is_none")]
	refresh_expires_in: Option<u64>,
	membership_id: String,
}

impl ExtraTokenFields for D2ExtraFields {}
