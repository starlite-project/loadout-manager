use serde::{Deserialize, Serialize};

use crate::{
	model::{util::BungieMembershipType, IconUrl},
	util::API_BASE,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfoCard {
	pub supplemental_display_name: String,
	pub icon_path: String,
	pub cross_save_override: i32,
	pub applicable_membership_types: Vec<BungieMembershipType>,
	pub is_public: bool,
	pub membership_type: BungieMembershipType,
    #[serde(with = "crate::util::values_as_strings")]
    pub membership_id: i64,
    pub bungie_global_display_name: String,
    pub bungie_global_display_name_code: Option<i16>
}

impl IconUrl for UserInfoCard {
	fn icon_url(&self) -> Result<url::Url, url::ParseError> {
		let mut base = url::Url::parse(API_BASE)?;

		base.set_path(&self.icon_path);

		Ok(base)
	}
}
