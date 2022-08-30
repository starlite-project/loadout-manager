#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::sync::atomic::{AtomicUsize, Ordering};

use app::{oauth::D2OAuthResponse, Result, StoreBuilder};
use tauri::api::http::ClientBuilder;
use tokio::runtime::Builder as RtBuilder;

static THREAD_ID: AtomicUsize = AtomicUsize::new(1);

fn main() -> Result<()> {
	let runtime = RtBuilder::new_multi_thread()
		.enable_all()
		.thread_name_fn(|| {
			let id = THREAD_ID.fetch_add(1, Ordering::SeqCst) + 1;
			let output = String::from("lm-pool-");
			output + &id.to_string()
		})
		.on_thread_stop(|| {
			THREAD_ID.fetch_sub(1, Ordering::SeqCst);
		})
		.build()?;

	tauri::async_runtime::set(runtime.handle().clone());

	let client = ClientBuilder::new().build()?;

	let store = StoreBuilder::new()
		.default(
			"auth_data".to_owned(),
			serde_json::to_value(D2OAuthResponse::default())?,
		)
		.build();

	tauri::Builder::default()
		.manage(client)
		.plugin(store)
		.invoke_handler(tauri::generate_handler![
			app::fetch::get_bungie_applications,
			app::oauth::get_authorization_code,
		])
		.run(tauri::generate_context!())?;
	Ok(())
}
