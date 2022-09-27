use oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl};
use reqwest::{
	header::{HeaderMap, HeaderValue},
	Client, ClientBuilder,
};
use url::Url;

use self::oauth::D2OAuthClient;
use crate::{
	fetch::IntoRequest,
	util::{API_BASE, API_KEY},
	Result,
};

pub mod oauth;
pub mod token;

#[derive(Debug, Clone)]
pub struct LoadoutClient {
	pub request_client: Client,
	oauth_client: D2OAuthClient,
}

impl LoadoutClient {
	pub fn new() -> Result<Self> {
		let mut default_headers = HeaderMap::new();

		default_headers.append("x-api-key", HeaderValue::from_str(API_KEY)?);

		let reqwest_client_builder = ClientBuilder::new()
			.default_headers(default_headers)
			.cookie_store(true);

		let oauth_client = D2OAuthClient::new(
			ClientId::new(crate::util::CLIENT_ID.to_owned()),
			Some(ClientSecret::new(crate::util::CLIENT_SECRET.to_owned())),
			AuthUrl::new("https://www.bungie.net/en/OAuth/Authorize/".to_owned())?,
			Some(TokenUrl::new(
				"https://www.bungie.net/Platform/App/OAuth/Token/".to_owned(),
			)?),
		);

		Ok(Self {
			request_client: reqwest_client_builder.build()?,
			oauth_client,
		})
	}

	pub fn oauth(&self) -> &D2OAuthClient {
		&self.oauth_client
	}

	pub fn from_route(
		&self,
		route: impl IntoRequest,
		token: String,
	) -> Result<reqwest::RequestBuilder> {
		let mut url: Url = API_BASE.parse()?;

		let query = route.query();

		let path = "Platform".to_owned() + route.to_string().as_str();

		url.set_path(path.as_str());

		url.set_query(query.as_deref());

		let builder = self.request_client.request(route.method(), url);

		Ok(builder.bearer_auth(token))
	}

	pub async fn make_oauth_request(
		&self,
		request: oauth2::HttpRequest,
	) -> Result<oauth2::HttpResponse, oauth2::reqwest::Error<reqwest::Error>> {
		let builder = self
			.request_client
			.request(request.method, request.url)
			.headers(request.headers)
			.body(request.body);

		let raw = builder
			.send()
			.await
			.map_err(oauth2::reqwest::Error::Reqwest)?;

		let status = raw.status();
		let headers = raw.headers().clone();
		let bytes = raw.bytes().await.map_err(oauth2::reqwest::Error::Reqwest)?;

		Ok(oauth2::HttpResponse {
			status_code: status,
			headers,
			body: bytes.to_vec(),
		})
	}
}
