use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
	pub application_id: i32,
	pub name: String,
	pub redirect_url: String,
	pub link: String,
	#[serde(with = "crate::util::values_as_strings")]
	pub scope: i64,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub origin: Option<String>,
	pub status: i32,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize_repr, Deserialize_repr)]
pub enum ApplicationStatus {
	None = 0,
	Private,
	Public,
	Disabled,
	Blocked,
}
