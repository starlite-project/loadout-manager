#![allow(dead_code)]

use std::fmt::{Display, Formatter, Result as FmtResult, Write};

use tauri::api::{self, http::HttpRequestBuilder};
use url::Url;

use crate::{
	model::util::{DestinyComponentType, BungieMembershipType},
	util::{API_BASE, API_KEY},
};

#[derive(Debug, Clone)]
pub enum AppRoute {
	ApiUsage(String),
	FirstParty,
}

impl Display for AppRoute {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("/App")?;
		match self {
			Self::ApiUsage(app_id) => {
				f.write_str("/ApiUsage/")?;
				f.write_str(&app_id)
			}
			Self::FirstParty => f.write_str("/FirstParty"),
		}
	}
}

impl IntoRequest for AppRoute {
	fn method(&self) -> &'static str {
		"GET"
	}

	fn query(&self) -> Option<String> {
		None
	}
}

#[derive(Debug, Clone)]
pub enum UserRoute {
	GetBungieNetUserById(i64),
	GetSanitizedPlatformDisplayNames(i64),
	GetCredentialTypesForTargetAccount(i64),
	GetAvailableThemes,
	GetMembershipDataById(i64, i32),
	GetMembershipDataForCurrentUser,
	GetMembershipFromHardLinkedCredential(u8, String),
	SearchByGlobalNamePost(i32),
}

impl Display for UserRoute {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("/User")?;

		match self {
			Self::GetBungieNetUserById(id) => {
				f.write_str("/GetBungieNetUserById/")?;
				Display::fmt(id, f)
			}
			Self::GetSanitizedPlatformDisplayNames(id) => {
				f.write_str("/GetSanitizedPlatformDisplayNames/")?;
				Display::fmt(id, f)
			}
			Self::GetCredentialTypesForTargetAccount(id) => {
				f.write_str("/GetCredentialTypesForTargetAccount/")?;
				Display::fmt(id, f)
			}
			Self::GetAvailableThemes => f.write_str("/GetAvailableThemes"),
			Self::GetMembershipDataById(id, kind) => {
				f.write_str("/GetMembershipsById/")?;
				Display::fmt(id, f)?;
				f.write_char('/')?;
				Display::fmt(kind, f)
			}
			Self::GetMembershipDataForCurrentUser => f.write_str("/GetMembershipsForCurrentUser"),
			Self::GetMembershipFromHardLinkedCredential(cr_type, cred) => {
				f.write_str("/GetMembershipFromHardLinkedCredential/")?;
				Display::fmt(cr_type, f)?;
				f.write_char('/')?;
				f.write_str(cred)
			}
			Self::SearchByGlobalNamePost(page) => {
				f.write_str("/Search/GlobalName/")?;
				Display::fmt(page, f)
			}
		}
	}
}

impl IntoRequest for UserRoute {
	fn method(&self) -> &'static str {
		if matches!(self, Self::SearchByGlobalNamePost(_)) {
			"POST"
		} else {
			"GET"
		}
	}

	fn query(&self) -> Option<String> {
		None
	}
}

#[derive(Debug, Clone)]
pub enum Destiny2Route {
	GetDestinyManifest,
	GetDestinyEntityDefinition(String, u32),
	GetLinkedProfiles(i64, BungieMembershipType, Option<bool>),
	GetProfile(i64, BungieMembershipType, Vec<DestinyComponentType>),
	GetCharacter(i64, i64, BungieMembershipType, Vec<DestinyComponentType>),
	GetItem(i64, i64, BungieMembershipType, Vec<DestinyComponentType>),
	TransferItem,
	PullFromPostmaster,
	EquipItem,
	EquipItems,
	SetItemLockState,
}

impl Display for Destiny2Route {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.write_str("/Destiny2")?;

		match self {
			Self::GetDestinyManifest => f.write_str("/Manifest/"),
			Self::GetDestinyEntityDefinition(entity_type, hash_identifier) => {
				f.write_str("/Manifest/")?;
				Display::fmt(entity_type, f)?;
				f.write_char('/')?;
				Display::fmt(hash_identifier, f)?;
				f.write_char('/')
			}
			Self::GetLinkedProfiles(membership_id, membership_type, _) => {
				f.write_char('/')?;
				Display::fmt(membership_type, f)?;
				f.write_str("/Profile/")?;
				Display::fmt(membership_id, f)?;
				f.write_str("/LinkedProfiles/")
			}
			Self::GetProfile(membership_id, membership_type, _) => {
				f.write_char('/')?;
				Display::fmt(membership_type, f)?;
				f.write_str("/Profile/")?;
				Display::fmt(membership_id, f)?;
				f.write_char('/')
			}
			Self::GetCharacter(character_id, destiny_membership_id, membership_type, _) => {
				f.write_char('/')?;
				Display::fmt(membership_type, f)?;
				f.write_str("/Profile/")?;
				Display::fmt(destiny_membership_id, f)?;
				f.write_str("/Character/")?;
				Display::fmt(character_id, f)?;
				f.write_char('/')
			}
			Self::GetItem(destiny_membership_id, item_instance_id, membership_type, _) => {
				f.write_char('/')?;
				Display::fmt(membership_type, f)?;
				f.write_str("/Profile/")?;
				Display::fmt(destiny_membership_id, f)?;
				f.write_str("/Item/")?;
				Display::fmt(item_instance_id, f)?;
				f.write_char('/')
			}
			Self::TransferItem => f.write_str("/Actions/Items/TransferItem/"),
			Self::PullFromPostmaster => f.write_str("/Actions/Items/PullFromPostmaster/"),
			Self::EquipItem => f.write_str("/Actions/Items/EquipItem/"),
			Self::EquipItems => f.write_str("/Actions/Items/EquipItems/"),
			Self::SetItemLockState => f.write_str("/Actions/Items/SetLockState/"),
		}
	}
}

impl IntoRequest for Destiny2Route {
	fn method(&self) -> &'static str {
		match self {
			Self::GetDestinyManifest
			| Self::GetDestinyEntityDefinition(..)
			| Self::GetLinkedProfiles(..)
			| Self::GetProfile(..)
			| Self::GetCharacter(..)
			| Self::GetItem(..) => "GET",
			Self::TransferItem
			| Self::PullFromPostmaster
			| Self::EquipItem
			| Self::EquipItems
			| Self::SetItemLockState => "POST",
		}
	}

	fn query(&self) -> Option<String> {
		match self {
			Self::GetDestinyManifest
			| Self::GetDestinyEntityDefinition(..)
			| Self::TransferItem
			| Self::PullFromPostmaster
			| Self::EquipItem
			| Self::EquipItems
			| Self::SetItemLockState => None,
			Self::GetLinkedProfiles(_, _, get_all_memberships) => {
				Some("getAllMemberships=".to_owned() + (*get_all_memberships)?.to_string().as_str())
			}
			Self::GetProfile(_, _, components)
			| Self::GetCharacter(_, _, _, components)
			| Self::GetItem(_, _, _, components) => {
				let components: &[DestinyComponentType] = &components[..];

				if components.is_empty() {
					panic!("no components were passed");
				}

				let stringified = components.iter().map(|x| x.to_string()).collect::<Vec<_>>();

				Some("components=".to_owned() + stringified.join(",").as_str())
			}
		}
	}
}

pub trait IntoRequest: Display {
	fn method(&self) -> &'static str;

	fn query(&self) -> Option<String>;

	fn into_request(&self) -> api::Result<api::http::HttpRequestBuilder> {
		let query = self.query();

		let mut uri = (API_BASE.to_owned() + &self.to_string()).parse::<Url>()?;

		uri.set_query(query.as_deref());

		let method = self.method();

		HttpRequestBuilder::new(method, uri)?.header("X-API-Key", API_KEY)
	}
}
