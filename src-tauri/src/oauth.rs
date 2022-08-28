use std::collections::HashMap;

use oauth2::{
	basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
	PkceCodeChallenge, TokenResponse, TokenUrl,
};
use tauri::{
	api::http::{Body, HttpRequestBuilder, ResponseType},
	Manager,
};
use tiny_http::{Response, Server};
use url::Url;

use super::fetch::CLIENT_ID;
use crate::{LoadoutState, Result};

#[cfg(windows)]
const PRIVATE_HTTPS_KEY: &[u8] = include_bytes!("..\\localhost-key.pem");
#[cfg(unix)]
const PRIVATE_HTTPS_KEY: &[u8] = include_bytes!("../localhost-key.pem");
#[cfg(windows)]
const LOCAL_CERT: &[u8] = include_bytes!("..\\localhost.pem");
#[cfg(unix)]
const LOCAL_CERT: &[u8] = include_bytes!("../localhost.pem");

const CLIENT_SECRET: &str = env!("CLIENT_SECRET");

#[tauri::command]
pub async fn begin_oauth(
	app_handle: tauri::AppHandle,
	state: tauri::State<'_, LoadoutState>,
) -> Result<()> {
	let client = BasicClient::new(
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

	let server = Server::https(
		"127.0.0.1:8000",
		tiny_http::SslConfig {
			certificate: LOCAL_CERT.to_vec(),
			private_key: PRIVATE_HTTPS_KEY.to_vec(),
		},
	)
	.unwrap();

	scope.open(auth_url.as_str(), None)?;

	let request = server.incoming_requests().next().unwrap();

	let response = Response::from_string("You may now close this tab");

	let url = ("https://localhost:8000".to_owned() + request.url()).parse::<Url>()?;

	let query_params = url
		.query_pairs()
		.map(|(key, value)| (key.into_owned(), value.into_owned()))
		.collect::<HashMap<String, String>>();

	let state_code = query_params
		.get("state")
		.expect("failed to find state, this is a big problem!");

	assert_eq!(state_code, csrf_token.secret());

	let auth_code = query_params
		.get("code")
		.expect("failed to find auth code, this is a big problem!");

	request.respond(response)?;

	let http = state.http();

	let token_result = client
		.exchange_code(AuthorizationCode::new(auth_code.clone()))
		.set_pkce_verifier(pkce_verifier)
		.request_async(move |req| make_request(http, req))
		.await?;

	let refresh_token = token_result
		.refresh_token()
		.expect("didn't receive refresh token, this is very bad!");

	let r = client
		.exchange_refresh_token(refresh_token)
		.request_async(move |req| make_request(http, req))
		.await?;

	dbg!(r);

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
