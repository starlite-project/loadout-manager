#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use anyhow::Result;

fn main() -> Result<()> {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![app::fetch::get_bungie_applications])
		.run(tauri::generate_context!()).expect("uh oh");
	Ok(())
}
