use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{de::DeserializeOwned, Serialize};
use tauri::api::http::{Client, HttpRequestBuilder, ResponseType};

use crate::{
	model::{Application, BungieResponse, GeneralUser},
	oauth::D2Token,
	plugins::Store,
	Result,
};

const API_BASE: &str = "https://bungie.net/Platform";

pub const API_KEY: &str = env!("API_KEY");

pub const CLIENT_ID: &str = env!("CLIENT_ID");

pub const CLIENT_SECRET: &str = env!("CLIENT_SECRET");

#[tauri::command]
pub async fn get_bungie_applications(
	state: tauri::State<'_, Client>,
) -> Result<BungieResponse<Vec<Application>>> {
	fetch(&*state, "/App/FirstParty", Method::Get).await
}

#[tauri::command]
pub async fn get_user(
	state: tauri::State<'_, Client>,
	store: tauri::State<'_, Store>,
) -> Result<BungieResponse<GeneralUser>> {
	todo!()
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
enum Method {
	Get,
	Post,
	Put,
	Delete,
	Patch,
	Head,
	Options,
	Connect,
	Trace,
}

impl Display for Method {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str(match self {
			Self::Get => "GET",
			Self::Post => "POST",
			Self::Put => "PUT",
			Self::Delete => "DELETE",
			Self::Patch => "PATCH",
			Self::Head => "HEAD",
			Self::Options => "OPTIONS",
			Self::Connect => "CONNECT",
			Self::Trace => "TRACE",
		})
	}
}

async fn fetch<T: Serialize + DeserializeOwned>(
	client: &Client,
	route: &str,
	method: Method,
) -> Result<BungieResponse<T>> {
	let request_builder = get_request_builder(route, method)?;

	let raw = client.send(request_builder).await?.bytes().await?.data;

	let res = serde_json::from_slice(&raw)?;

	Ok(res)
}

async fn auth_fetch<T: Serialize + DeserializeOwned>(
	client: &Client,
	storage: &Store,
	route: &str,
	method: Method,
) -> Result<BungieResponse<T>> {
	let auth_data: D2Token =
		serde_json::from_value(storage.get("auth_data").await.expect("auth data not saved"))?;

	let token = auth_data.access_token;

	let request_builder = get_request_builder(route, method)?
		.header("Authorization", "Bearer ".to_owned() + token.as_str())?;

	let raw = client.send(request_builder).await?.bytes().await?.data;

	let res = serde_json::from_slice(&raw)?;

	Ok(res)
}

fn get_request_builder(route: &str, method: Method) -> Result<HttpRequestBuilder> {
	let route = API_BASE.to_owned() + route;
	Ok(HttpRequestBuilder::new(method.to_string(), route)?
		.header("X-API-Key", API_KEY)?
		.response_type(ResponseType::Json))
}
