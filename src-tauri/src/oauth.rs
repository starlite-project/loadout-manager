use std::{
	fmt::{Display, Formatter, Result as FmtResult},
	time::{Duration, SystemTime},
};

use oauth2::{
	basic::{
		BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse,
		BasicTokenType,
	},
	AuthUrl, Client as OAuth2Client, ClientId, ClientSecret, CsrfToken, ExtraTokenFields,
	PkceCodeChallenge, StandardRevocableToken, StandardTokenResponse, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use tauri::{
	api::http::{Body, Client, HttpRequestBuilder, ResponseType},
	Manager,
};

use super::{fetch::CLIENT_ID, Result};

const CLIENT_SECRET: &str = env!("CLIENT_SECRET");

#[tauri::command]
pub async fn get_authorization_code(
	app_handle: tauri::AppHandle,
	state: tauri::State<'_, Client>,
) -> Result<D2OAuthResponse> {
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

	let scope = app_handle.shell_scope();

	// let server = Server::https(
	// 	"127.0.0.1:8000",
	// 	tiny_http::SslConfig {
	// 		certificate: LOCAL_CERT.to_vec(),
	// 		private_key: PRIVATE_HTTPS_KEY.to_vec(),
	// 	},
	// )
	// .unwrap();

	scope.open(auth_url.as_str(), None)?;

	// let request = server.incoming_requests().next().unwrap();

	// let response = Response::from_string("You may now close this tab");

	// let url = ("https://localhost:8000".to_owned() + request.url()).parse::<Url>()?;

	// let query_params = url
	// 	.query_pairs()
	// 	.map(|(key, value)| (key.into_owned(), value.into_owned()))
	// 	.collect::<HashMap<String, String>>();

	// let state_code = query_params
	// 	.get("state")
	// 	.expect("failed to find state, this is a big problem!");

	// assert_eq!(state_code, csrf_token.secret());

	// let auth_code = query_params
	// 	.get("code")
	// 	.expect("failed to find auth code, this is a big problem!");

	// request.respond(response)?;

	// let http = &*state;

	// let token_result = client
	// 	.exchange_code(AuthorizationCode::new(auth_code.clone()))
	// 	.set_pkce_verifier(pkce_verifier)
	// 	.request_async(move |req| make_request(http, req))
	// 	.await?;

	// Ok(token_result.try_into()?)

	todo!()
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
pub struct D2OAuthResponse {
	pub access_token: String,
	pub expires_in: SystemTime,
	pub refresh_token: String,
	pub refresh_expires_in: SystemTime,
	pub membership_id: String,
}

impl Default for D2OAuthResponse {
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

impl TryFrom<StandardTokenResponse<D2ExtraFields, BasicTokenType>> for D2OAuthResponse {
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

		Ok(D2OAuthResponse {
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
