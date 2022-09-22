use serde::{de::DeserializeOwned, Serialize};

use self::routing::{AppRoute, UserRoute};
use crate::{
	model::{Application, BungieResponse, GeneralUser},
	LoadoutClient, Result,
};

mod routing;

pub use self::routing::IntoRequest;

#[tauri::command]
pub async fn get_bungie_applications(
	http: tauri::State<'_, LoadoutClient>,
	token: String,
) -> Result<BungieResponse<Vec<Application>>> {
	basic_fetch(&*http, token, AppRoute::FirstParty).await
}

#[tauri::command]
pub async fn get_current_user(
	http: tauri::State<'_, LoadoutClient>,
	token: String,
	membership_id: String,
) -> Result<BungieResponse<GeneralUser>> {
	basic_fetch(
		&*http,
		token,
		UserRoute::GetBungieNetUserById(membership_id),
	)
	.await
}

#[allow(dead_code)]
async fn basic_fetch<T: Serialize + DeserializeOwned>(
	client: &LoadoutClient,
	token: String,
	route: impl IntoRequest,
) -> Result<BungieResponse<T>> {
	let request = client.from_route(route, token)?;
	// let raw = client.send(request_builder).await?.bytes().await?.data;

	let raw = request.send().await?.bytes().await?;

	let res = serde_json::from_slice(&raw)?;

	Ok(res)
}
