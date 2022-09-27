use std::time::Duration;

use oauth2::{
	basic::{
		BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
		BasicTokenType,
	},
	AuthorizationCode, Client as OAuth2Client, CsrfToken, ExtraTokenFields, PkceCodeChallenge,
	RefreshToken, StandardRevocableToken, StandardTokenResponse,
};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tokio::time::sleep;
use url::Url;

use super::token::AuthTokens;
use crate::{LoadoutClient, Result};

const REDIRECT_SERVER: &str = env!("SERVER_LOCATION");

#[tauri::command]
pub async fn get_authorization_code(
	app_handle: tauri::AppHandle,
	http: tauri::State<'_, LoadoutClient>,
) -> Result<AuthTokens> {
	let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

	let (auth_url, csrf_token) = http
		.oauth()
		.authorize_url(CsrfToken::new_random)
		.set_pkce_challenge(pkce_challenge)
		.url();

	dbg!(REDIRECT_SERVER);

	let mut location = Url::parse(REDIRECT_SERVER)?;

	if location.domain().is_none() {
		// we know it's internal (aka 192.168.x.x)
		_ = location.set_port(Some(3030));
	}

	location.set_path("retrieval");

	let query = "state=".to_owned() + csrf_token.secret().as_str();

	location.set_query(Some(&query));

	let scope = app_handle.shell_scope();

	scope.open(auth_url.as_str(), None)?;

	let code = loop {
		let value = http
			.request_client
			.get(location.clone())
			.send()
			.await?
			.json::<Option<String>>()
			.await?;

		if let Some(code) = value {
			break code;
		} else {
			sleep(Duration::from_secs(5)).await;
		}
	};

	log::debug!("received code {} from bungie", code);

	let token_result = http
		.oauth()
		.exchange_code(AuthorizationCode::new(code.to_owned()))
		.set_pkce_verifier(pkce_verifier)
		.request_async(|req| http.make_oauth_request(req))
		.await?;

	Ok(AuthTokens::from_oauth_response(token_result).unwrap())
}

#[tauri::command]
pub async fn refresh_token(
	http: tauri::State<'_, LoadoutClient>,
	token: AuthTokens,
) -> Result<AuthTokens> {
	let refresh_token = RefreshToken::new(
		token
			.refresh_token
			.expect("no refresh token present")
			.value()
			.to_owned(),
	);

	let new_auth_data = http
		.oauth()
		.exchange_refresh_token(&refresh_token)
		.request_async(|req| http.make_oauth_request(req))
		.await?;

	Ok(AuthTokens::from_oauth_response(new_auth_data).unwrap())
}

pub(super) type D2OAuthResponse = StandardTokenResponse<D2ExtraFields, BasicTokenType>;

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
	#[serde(with = "crate::util::values_as_strings")]
	membership_id: i64,
}

impl D2ExtraFields {
	pub fn expires_in(&self) -> Option<Duration> {
		self.refresh_expires_in.map(Duration::from_secs)
	}

	pub fn membership_id(&self) -> i64 {
		self.membership_id
	}
}

impl ExtraTokenFields for D2ExtraFields {}
