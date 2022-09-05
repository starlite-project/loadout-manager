#![allow(dead_code)]

use std::fmt::{Display, Formatter, Result as FmtResult, Write};

use tauri::api::http::HttpRequestBuilder;
use url::Url;

use crate::util::{API_BASE, API_KEY};

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
}

pub trait IntoRequest: Display {
	fn method(&self) -> &'static str;

	fn into_request(&self) -> tauri::api::Result<tauri::api::http::HttpRequestBuilder> {
		let uri = (API_BASE.to_owned() + &self.to_string()).parse::<Url>()?;

		let method = self.method();

		HttpRequestBuilder::new(method, uri)?.header("X-API-Key", API_KEY)
	}
}
