#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::sync::atomic::{AtomicUsize, Ordering};

use app::{
	oauth::D2Token,
	plugins::{fern::colors, LogLevel, LogTarget, LoggerBuilder, RotationStrategy, StoreBuilder},
	LoadoutClient, Result,
};
use tauri::{Manager, CustomMenuItem, Submenu, Menu, MenuItem};
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

	let store = StoreBuilder::new()
		.default(
			"auth_data".to_owned(),
			serde_json::to_value(D2Token::default())?,
		)
		.build();

	let log = LoggerBuilder::new()
		.level(LogLevel::Trace)
		.target(LogTarget::Webview)
		.with_colors(colors::ColoredLevelConfig {
			error: colors::Color::Red,
			warn: colors::Color::Yellow,
			info: colors::Color::White,
			debug: colors::Color::Cyan,
			trace: colors::Color::Magenta,
		})
		.rotation_strategy(RotationStrategy::KeepAll)
		.build();

	let menu = {
		let quit = CustomMenuItem::new("quit", "Quit");
		let close = CustomMenuItem::new("close", "Close");
		let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
		Menu::new().add_native_item(MenuItem::Copy)
		.add_item(CustomMenuItem::new("hide", "Hide")).add_submenu(submenu)
	};

	tauri::Builder::default()
		.manage(LoadoutClient::new()?)
		.plugin(store)
		.plugin(log)
		.invoke_handler(tauri::generate_handler![
			app::fetch::get_bungie_applications,
			app::fetch::get_current_user,
			app::oauth::get_authorization_code,
			app::oauth::is_token_valid,
			app::oauth::refresh_token,
			app::oauth::is_token_refreshable,
			app::oauth::delete_token,
		])
		.setup(|app| {
			let keys = app.windows().into_keys().collect::<Vec<_>>();
			dbg!(keys);
			Ok(())
		})
		.menu(menu)
		.on_menu_event(|event| {
			match event.menu_item_id() {
				"quit" => {
					std::process::exit(0);
				}
				"close" => {
					event.window().close().unwrap();
				}
				_ => {}
			}
		})
		.run(tauri::generate_context!())?;
	Ok(())
}
