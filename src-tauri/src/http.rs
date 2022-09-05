use futures_util::Future;
use oauth2::{
	basic::{BasicErrorResponseType, BasicTokenType},
	AuthUrl, AuthorizationCode, AuthorizationRequest, ClientId, ClientSecret, CodeTokenRequest,
	CsrfToken, RefreshToken, RefreshTokenRequest, StandardErrorResponse, StandardTokenResponse,
	TokenUrl,
};
use tauri::api::http::{Client, ClientBuilder, HttpRequestBuilder, Response};

use crate::{
	oauth::{D2ExtraFields, D2OAuthClient},
	Result,
};

#[derive(Debug, Clone)]
pub struct LoadoutClient {
	request_client: Client,
	oauth_client: D2OAuthClient,
}

impl LoadoutClient {
	pub fn new() -> Result<Self> {
		let request_client_builder = ClientBuilder::new();

		let oauth_client = D2OAuthClient::new(
			ClientId::new(crate::util::CLIENT_ID.to_owned()),
			Some(ClientSecret::new(crate::util::CLIENT_SECRET.to_owned())),
			AuthUrl::new("https://www.bungie.net/en/OAuth/Authorize/".to_owned())?,
			Some(TokenUrl::new(
				"https://www.bungie.net/Platform/App/OAuth/Token/".to_owned(),
			)?),
		);

		Ok(Self {
			request_client: request_client_builder.build()?,
			oauth_client,
		})
	}

	pub fn send(
		&self,
		request: HttpRequestBuilder,
	) -> impl Future<Output = tauri::api::Result<Response>> + '_ {
		self.request_client.send(request)
	}

	pub fn authorize_url<S>(&self, state_fn: S) -> AuthorizationRequest<'_>
	where
		S: FnOnce() -> CsrfToken,
	{
		self.oauth_client.authorize_url(state_fn)
	}

	pub fn exchange_code(
		&self,
		code: AuthorizationCode,
	) -> CodeTokenRequest<
		StandardErrorResponse<BasicErrorResponseType>,
		StandardTokenResponse<D2ExtraFields, BasicTokenType>,
		BasicTokenType,
	> {
		self.oauth_client.exchange_code(code)
	}

	pub fn exchange_refresh_token<'a>(
		&'a self,
		token: &'a RefreshToken,
	) -> RefreshTokenRequest<
		'a,
		StandardErrorResponse<BasicErrorResponseType>,
		StandardTokenResponse<D2ExtraFields, BasicTokenType>,
		BasicTokenType,
	> {
		self.oauth_client.exchange_refresh_token(token)
	}
}
