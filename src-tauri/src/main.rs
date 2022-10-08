#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::sync::atomic::{AtomicUsize, Ordering};

use app::{
	plugins::{
		fern::colors, set_shadow, LogLevel, LoggerBuilder, RotationStrategy, WindowStateBuilder,
	},
	LoadoutClient, Result,
};
#[cfg(debug_assertions)]
use tauri::Manager;
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

	let log = LoggerBuilder::new()
		.level(if cfg!(debug_assertions) {
			LogLevel::Trace
		} else {
			LogLevel::Debug
		})
		.with_colors(colors::ColoredLevelConfig {
			error: colors::Color::Red,
			warn: colors::Color::Yellow,
			info: colors::Color::White,
			debug: colors::Color::Cyan,
			trace: colors::Color::Magenta,
		})
		.rotation_strategy(RotationStrategy::KeepAll)
		.build();

	tauri::Builder::default()
		.manage(LoadoutClient::new()?)
		.plugin(log)
		.plugin(WindowStateBuilder::default().build())
		.invoke_handler(tauri::generate_handler![
			app::fetch::get_bungie_applications,
			app::fetch::get_current_user,
			app::http::oauth::get_authorization_code,
			app::http::oauth::refresh_token,
		])
		.setup(|app| {
			let window = app.get_window("main").unwrap();
			#[cfg(debug_assertions)]
			window.open_devtools();

			_ = set_shadow(&window, true);

			Ok(())
		})
		.run(tauri::generate_context!())?;
	Ok(())
}
