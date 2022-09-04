use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralUser {
    #[serde(deserialize_with = "crate::util::deserialize_string_as")]
    pub membership_id: i64,
    pub unique_name: String,
    pub normalized_name: String,
    pub display_name: String,
    pub profile_picture: i32,
    pub profile_theme: i32,
    pub user_title: i32,
    #[serde(deserialize_with = "crate::util::deserialize_string_as")]
    pub success_message_flags: i64,
    pub is_deleted: bool,
    pub about: String,
    pub psn_display_name: String,
    pub xbox_display_name: String,
    pub fb_display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_activity: Option<bool>,
    pub locale: String,
}