use serde::{de::DeserializeOwned, Serialize};

use self::routing::{AppRoute, UserRoute};
use crate::{
	http::oauth::D2Token,
	model::{Application, BungieResponse, GeneralUser},
	plugins::Store,
	LoadoutClient, Result,
};

mod routing;

pub use self::routing::IntoRequest;

#[tauri::command]
pub async fn get_bungie_applications(
	http: tauri::State<'_, LoadoutClient>,
	store: tauri::State<'_, Store>,
) -> Result<BungieResponse<Vec<Application>>> {
	basic_fetch(&*http, &*store, AppRoute::FirstParty).await
}

#[tauri::command]
pub async fn get_current_user(
	http: tauri::State<'_, LoadoutClient>,
	store: tauri::State<'_, Store>,
) -> Result<BungieResponse<GeneralUser>> {
	let auth_data = store.get("auth_data").await.expect("not logged in");

	let membership_id = serde_json::from_value::<D2Token>(auth_data)?.membership_id;

	basic_fetch(
		&*http,
		&*store,
		UserRoute::GetBungieNetUserById(membership_id),
	)
	.await
}

#[allow(dead_code)]
async fn basic_fetch<T: Serialize + DeserializeOwned>(
	client: &LoadoutClient,
	storage: &Store,
	route: impl IntoRequest,
) -> Result<BungieResponse<T>> {
	let auth_data: D2Token =
		serde_json::from_value(storage.get("auth_data").await.expect("auth data not saved"))?;

	let token = auth_data.access_token;

	let request = client.from_route(route, token)?;
	// let raw = client.send(request_builder).await?.bytes().await?.data;

	let raw = request.send().await?.bytes().await?;

	let res = serde_json::from_slice(&raw)?;

	Ok(res)
}
