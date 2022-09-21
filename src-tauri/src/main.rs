#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::sync::atomic::{AtomicUsize, Ordering};

use app::{
	http::oauth::D2Token,
	plugins::{
		fern::colors, LogLevel, LogTarget, LoggerBuilder, RotationStrategy, Store, StoreBuilder,
	},
	LoadoutClient, Result,
};
use oauth2::RefreshToken;
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

	tauri::Builder::default()
		.manage(LoadoutClient::new()?)
		.plugin(store)
		.plugin(log)
		.invoke_handler(tauri::generate_handler![
			app::fetch::get_bungie_applications,
			app::fetch::get_current_user,
			app::http::oauth::get_authorization_code,
			app::http::oauth::is_token_valid,
			app::http::oauth::refresh_token,
			app::http::oauth::is_token_refreshable,
			app::http::oauth::logged_in,
		])
		.setup(|app| {
			tauri::async_runtime::block_on(async move {
				let storage = app.state::<Store>();

				let auth_data = storage.get("auth_data").await;

				if let Some(raw_value) = auth_data {
					let json = serde_json::from_value::<D2Token>(raw_value)?;

					if json.is_valid() {
						return Ok(());
					} else if json.is_refreshable() {
						let refresh_token = RefreshToken::new(json.refresh_token);

						let http = app.state::<LoadoutClient>();
						let new_auth_data = http
							.oauth()
							.exchange_refresh_token(&refresh_token)
							.request_async(|req| http.make_oauth_request(req))
							.await?;

						let new_token = D2Token::try_from(new_auth_data)?;

						storage
							.insert("auth_data".to_owned(), serde_json::to_value(new_token)?)
							.await;
					}
				}

				Ok::<(), Box<dyn std::error::Error>>(())
			})
		})
		.run(tauri::generate_context!())?;
	Ok(())
}
