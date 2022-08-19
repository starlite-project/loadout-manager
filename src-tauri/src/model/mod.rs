use serde::{Deserialize, Serialize};

mod application;

pub use self::application::{Application, ApplicationStatus};

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
