#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::sync::atomic::{AtomicUsize, Ordering};

use app::{
	fetch::{CLIENT_ID, CLIENT_SECRET},
	oauth::{D2OAuthClient, D2Token},
	plugins::{fern::colors, LogLevel, LogTarget, LoggerBuilder, RotationStrategy, StoreBuilder},
	Result,
};
use oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl};
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

	let oauth_client = D2OAuthClient::new(
		ClientId::new(CLIENT_ID.to_owned()),
		Some(ClientSecret::new(CLIENT_SECRET.to_owned())),
		AuthUrl::new("https://www.bungie.net/en/oauth/authorize/".to_owned())?,
		Some(TokenUrl::new(
			"https://www.bungie.net/Platform/App/OAuth/Token/".to_owned(),
		)?),
	);

	tauri::async_runtime::set(runtime.handle().clone());

	let client = ClientBuilder::new().build()?;

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
		.manage(client)
		.manage(oauth_client)
		.plugin(store)
		.plugin(log)
		.invoke_handler(tauri::generate_handler![
			app::fetch::get_bungie_applications,
			app::oauth::get_authorization_code,
		])
		.run(tauri::generate_context!())?;
	Ok(())
}
