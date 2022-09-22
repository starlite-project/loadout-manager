use serde::{de::DeserializeOwned, Serialize};

use self::routing::{AppRoute, UserRoute};
use crate::{
	model::{Application, BungieResponse, GeneralUser},
	LoadoutClient, Result, http::token::AuthTokens,
};

mod routing;

pub use self::routing::IntoRequest;

#[tauri::command]
pub async fn get_bungie_applications(
	http: tauri::State<'_, LoadoutClient>,
	token: AuthTokens,
) -> Result<BungieResponse<Vec<Application>>> {
	basic_fetch(&*http, token, AppRoute::FirstParty).await
}

#[tauri::command]
pub async fn get_current_user(
	http: tauri::State<'_, LoadoutClient>,
	token: AuthTokens,
) -> Result<BungieResponse<GeneralUser>> {
	let route = UserRoute::GetBungieNetUserById(token.bungie_membership_id);
	basic_fetch(
		&*http,
		token,
		route,
	)
	.await
}

#[allow(dead_code)]
async fn basic_fetch<T: Serialize + DeserializeOwned>(
	client: &LoadoutClient,
	token: AuthTokens,
	route: impl IntoRequest,
) -> Result<BungieResponse<T>> {
	let request = client.from_route(route, token.access_token.value().to_owned())?;

	let raw = request.send().await?.bytes().await?;

	let res = serde_json::from_slice(&raw)?;

	Ok(res)
}
