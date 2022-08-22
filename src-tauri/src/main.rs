#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::collections::HashMap;

use app::{LoadoutState, Result};
use serde_json::{Number as JsonNumber, Value as JsonValue};
use storage::{PluginBuilder, StoreBuilder};
use tauri::api::dialog;

fn main() -> Result<()> {
	let mut defaults = HashMap::new();
	defaults.insert("access_token".to_owned(), JsonValue::String("".to_owned()));
	defaults.insert(
		"token_type".to_owned(),
		JsonValue::String("Bearer".to_owned()),
	);
	defaults.insert(
		"expires_in".to_owned(),
		JsonValue::Number(JsonNumber::from(0_u64)),
	);
	defaults.insert("refresh_token".to_owned(), JsonValue::String("".to_owned()));
	defaults.insert(
		"refresh_expires_in".to_owned(),
		JsonValue::String("".to_owned()),
	);
	defaults.insert("membership_id".to_owned(), JsonValue::String("".to_owned()));

	let oauth = StoreBuilder::new(".oauth_data".parse()?)
		.defaults(defaults)
		.build();
	if let Err(e) = tauri::Builder::default()
		.plugin(PluginBuilder::default().stores([oauth]).freeze().build())
		.manage(LoadoutState::new()?)
		.invoke_handler(tauri::generate_handler![
			app::fetch::get_bungie_applications,
			app::fetch::begin_oauth,
		])
		.on_page_load(|_, payload| println!("{:?}", payload))
		.register_uri_scheme_protocol("loadout-manager", |_, _| panic!())
		.run(tauri::generate_context!())
	{
		let error_message = "An error has occurred: ".to_owned() + e.to_string().as_str();
		let dialog = dialog::MessageDialogBuilder::new("Error", error_message)
			.buttons(dialog::MessageDialogButtons::Ok)
			.kind(dialog::MessageDialogKind::Error);

		dialog.show(|_| std::process::exit(1));
	}
	Ok(())
}
