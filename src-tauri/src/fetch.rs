use std::fmt::{Display, Formatter, Result as FmtResult};

use serde::{de::DeserializeOwned, Serialize};
use tauri::api::http::{Client, HttpRequestBuilder, ResponseType};

use crate::{
	model::{Application, BungieResponse},
	LoadoutState, Result,
};

const API_BASE: &str = "https://bungie.net/Platform";

const API_KEY: &str = env!("API_KEY");

pub const CLIENT_ID: &str = env!("CLIENT_ID");

#[tauri::command]
pub async fn get_bungie_applications(
	state: tauri::State<'_, LoadoutState>,
) -> Result<BungieResponse<Vec<Application>>> {
	fetch(state.http(), "/App/FirstParty", Method::Get).await
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
	let route = API_BASE.to_owned() + route;

	let request_builder = HttpRequestBuilder::new(method.to_string(), route)?
		.header("X-API-Key", API_KEY)?
		.response_type(ResponseType::Json);

	let raw = client.send(request_builder).await?.bytes().await?.data;

	let res = serde_json::from_slice(&raw)?;

	Ok(res)
}
