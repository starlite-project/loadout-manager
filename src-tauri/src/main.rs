#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use anyhow::Result;
use app::LoadoutState;

fn main() -> Result<()> {
	tauri::Builder::default()
		.manage(LoadoutState::new()?)
		.invoke_handler(tauri::generate_handler![
			app::fetch::get_bungie_applications
		])
		.run(tauri::generate_context!())?;
	Ok(())
}
