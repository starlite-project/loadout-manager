use serde::{Deserialize, Serialize};

use crate::{
	model::{user::UserInfoCard, util::BungieMembershipType, IconUrl},
	util::API_BASE,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyProfileUserInfoCard {
	pub is_overridden: bool,
	pub is_cross_save_primary: bool,
	pub supplemental_display_name: String,
	pub icon_path: String,
	pub cross_save_override: i32,
	pub applicable_membership_types: Vec<BungieMembershipType>,
	pub is_public: bool,
	pub membership_type: BungieMembershipType,
	#[serde(with = "crate::util::values_as_strings")]
	pub membership_id: i64,
	pub display_name: String,
	pub bungie_global_display_name: String,
}

impl IconUrl for DestinyProfileUserInfoCard {
	fn icon_url(&self) -> Result<url::Url, url::ParseError> {
		let mut base = url::Url::parse(API_BASE)?;

		base.set_path(&self.icon_path);

		Ok(base)
	}
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinyLinkedProfilesResponse {
	pub profiles: Vec<DestinyProfileUserInfoCard>,
	pub bnet_membership: UserInfoCard,
}
