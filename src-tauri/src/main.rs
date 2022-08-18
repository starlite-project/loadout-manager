#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use anyhow::Result;
use dotenv::dotenv;

fn main() -> Result<()> {
	dotenv()?;
	tauri::Builder::default().run(tauri::generate_context!())?;
	Ok(())
}
