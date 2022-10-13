use serde::{Deserialize, Serialize};

mod application;
mod destiny;
mod user;
pub mod util;

pub use self::{application::*, destiny::*, user::*};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BungieResponse<T> {
	pub response: T,
	pub error_code: i32,
	pub throttle_seconds: i32,
	pub error_status: String,
	pub message: String,
	pub message_data: std::collections::HashMap<String, String>,
	pub detailed_error_trace: Option<String>,
}

pub trait IconUrl {
	fn icon_url(&self) -> Result<url::Url, url::ParseError>;
}
