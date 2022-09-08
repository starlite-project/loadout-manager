use serde::{de::DeserializeOwned, Serialize};

use self::routing::{AppRoute, IntoRequest, UserRoute};
use crate::{
	model::{Application, BungieResponse, GeneralUser},
	oauth::D2Token,
	plugins::Store,
	LoadoutClient, Result,
};

mod routing;

#[tauri::command]
pub async fn get_bungie_applications(
	http: tauri::State<'_, LoadoutClient>,
) -> Result<BungieResponse<Vec<Application>>> {
	basic_fetch(&*http, AppRoute::FirstParty).await
}

#[tauri::command]
pub async fn get_current_user(
	http: tauri::State<'_, LoadoutClient>,
	store: tauri::State<'_, Store>,
) -> Result<BungieResponse<GeneralUser>> {
	let auth_data = store.get("auth_data").await.expect("not logged in");

	let membership_id = serde_json::from_value::<D2Token>(auth_data)?.membership_id;

	basic_fetch(&*http, UserRoute::GetBungieNetUserById(membership_id)).await
}

async fn basic_fetch<T: Serialize + DeserializeOwned>(
	client: &LoadoutClient,
	route: impl IntoRequest,
) -> Result<BungieResponse<T>> {
	let request_builder = route.into_request()?;

	let raw = client.send(request_builder).await?.bytes().await?.data;

	let res = serde_json::from_slice(&raw)?;

	Ok(res)
}

#[allow(dead_code)]
async fn basic_auth_fetch<T: Serialize + DeserializeOwned>(
	client: &LoadoutClient,
	storage: &Store,
	route: impl IntoRequest,
) -> Result<BungieResponse<T>> {
	let auth_data: D2Token =
		serde_json::from_value(storage.get("auth_data").await.expect("auth data not saved"))?;

	let token = auth_data.access_token;

	let request_builder = route
		.into_request()?
		.header("Authorization", "Bearer ".to_owned() + token.as_str())?;

	let raw = client.send(request_builder).await?.bytes().await?.data;

	let res = serde_json::from_slice(&raw)?;

	Ok(res)
}
