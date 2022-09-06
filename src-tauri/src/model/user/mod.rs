use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralUser {
    #[serde(with = "crate::util::values_as_strings")]
    pub membership_id: i64,
    pub unique_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_name: Option<String>,
    pub display_name: String,
    pub profile_picture: i32,
    pub profile_theme: i32,
    pub user_title: i32,
    #[serde(with = "crate::util::values_as_strings")]
    pub success_message_flags: i64,
    pub is_deleted: bool,
    pub about: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psn_display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xbox_display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fb_display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_activity: Option<bool>,
    pub locale: String,
}