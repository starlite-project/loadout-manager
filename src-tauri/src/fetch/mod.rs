use std::time::Duration;

use serde::{de::DeserializeOwned, Serialize};

use self::routing::{AppRoute, Destiny2Route, UserRoute};
use crate::{
	http::token::AuthTokens,
	model::{
		util::BungieMembershipType, Application, BungieResponse, DestinyLinkedProfilesResponse,
		GeneralUser,
	},
	LoadoutClient,
};

mod error;
mod routing;

pub use self::{error::FetchError, routing::IntoRequest};

pub async fn get_linked_profiles(
	http: tauri::State<'_, LoadoutClient>,
	token: AuthTokens,
) -> Result<DestinyLinkedProfilesResponse, FetchError> {
	let route = Destiny2Route::GetLinkedProfiles(
		token.bungie_membership_id,
		BungieMembershipType::BungieNext,
		None,
	);
	basic_fetch(&*http, token, route).await
}

#[tauri::command]
pub async fn get_bungie_applications(
	http: tauri::State<'_, LoadoutClient>,
	token: AuthTokens,
) -> Result<Vec<Application>, FetchError> {
	basic_fetch(&*http, token, AppRoute::FirstParty).await
}

#[tauri::command]
pub async fn get_current_user(
	http: tauri::State<'_, LoadoutClient>,
	token: AuthTokens,
) -> Result<GeneralUser, FetchError> {
	let route = UserRoute::GetBungieNetUserById(token.bungie_membership_id);
	basic_fetch(&*http, token, route).await
}

#[allow(dead_code)]
async fn basic_fetch<T: Serialize + DeserializeOwned>(
	client: &LoadoutClient,
	token: AuthTokens,
	route: impl IntoRequest,
) -> Result<T, FetchError> {
	let request = client.from_route(route, token.access_token.value().to_owned())?;

	let raw = request.send().await?.bytes().await?;

	let res = serde_json::from_slice::<BungieResponse<T>>(&raw)
		.expect("failed to deserialize bungie data, this is bad!");

	if res.error_code != 1 {
		return Err(FetchError::Bungie {
			code: res.error_code,
			message: res.error_status,
			detailed_error_trace: res.detailed_error_trace,
		});
	}

	if res.throttle_seconds > 0 {
		tokio::time::sleep(Duration::from_secs(res.throttle_seconds as u64)).await;
	}

	Ok(res.response)
}
