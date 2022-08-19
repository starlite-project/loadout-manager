use anyhow::Result;
use once_cell::sync::OnceCell;
use reqwest::{Client, Method};
use serde::{de::DeserializeOwned, Serialize};

use crate::model::{Application, BungieResponse};

static CLIENT: OnceCell<Client> = OnceCell::new();

const API_BASE: &str = "https://bungie.net/Platform";

const API_KEY: &str = env!("API_KEY");

#[tauri::command]
pub async fn get_bungie_applications() -> Result<BungieResponse<Vec<Application>>, String> {
	fetch("/App/FirstParty", Method::GET)
		.await
		.map_err(|x| x.to_string())
}

async fn fetch<T: Serialize + DeserializeOwned>(
	route: &str,
	method: Method,
) -> Result<BungieResponse<T>> {
	let client = CLIENT.get_or_init(Client::new);

	let route = API_BASE.to_owned() + route;

	let request_builder = client.request(method, route);

	let res = request_builder
		.header("X-API-Key", API_KEY)
		.send()
		.await?
		.json()
		.await?;

	Ok(res)
}
