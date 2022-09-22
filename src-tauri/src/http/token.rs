use chrono::{DateTime, Utc};
use oauth2::TokenResponse;
use serde::{Deserialize, Serialize};

use super::oauth::D2OAuthResponse;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthTokens {
	pub access_token: Token,
	pub refresh_token: Option<Token>,
	pub bungie_membership_id: String,
}

impl AuthTokens {
	pub fn from_oauth_response(response: D2OAuthResponse) -> Option<Self> {
		let access_token = {
			let raw_token = response.access_token().secret().clone();
			let expires = Utc::now()
				.checked_add_signed(chrono::Duration::from_std(response.expires_in()?).ok()?)?;
			let name = TokenType::Access;

			Token {
				value: raw_token,
				expires,
				name,
			}
		};

		let refresh_token = {
			let raw_token = response.refresh_token()?.secret().clone();
			let expires = Utc::now().checked_add_signed(
				chrono::Duration::from_std(response.extra_fields().expires_in()?).ok()?,
			)?;
			let name = TokenType::Refresh;

			Some(Token {
				value: raw_token,
				expires,
				name,
			})
		};

		Some(Self {
			refresh_token,
			access_token,
			bungie_membership_id: response.extra_fields().membership_id().to_string(),
		})
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
	value: String,
	expires: DateTime<Utc>,
	name: TokenType,
}

impl Token {
	pub fn value(&self) -> &str {
		self.value.as_str()
	}

	pub fn expires_in(&self) -> i64 {
		self.expires.timestamp()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum TokenType {
	Access,
	Refresh,
}
