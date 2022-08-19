use std::env;

use anyhow::Result;
use tauri_build::Attributes;

fn main() -> Result<()> {
	dotenv::dotenv()?;
	let api_key = env::var("API_KEY").ok();
	if api_key.is_none() {
		panic!();
	}
	println!("rustc-env=API_KEY={}", api_key.unwrap());
	tauri_build::try_build(Attributes::default())?;

	Ok(())
}
