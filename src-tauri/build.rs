use std::env;

fn main() {
	dotenv::dotenv().ok();
	let api_key = env::var("API_KEY").unwrap();
	let client_id = env::var("CLIENT_ID").unwrap();
	let client_secret = env::var("CLIENT_SECRET").unwrap();
	let redirect_response_server = env::var("SERVER_LOCATION").unwrap_or("wss://starlight-loadout-server.com/socket".to_owned());
	println!("cargo:rustc-env=API_KEY={}", api_key);
	println!("cargo:rustc-env=CLIENT_ID={}", client_id);
	println!("cargo:rustc-env=CLIENT_SECRET={}", client_secret);
	println!("cargo:rustc-env=SERVER_LOCATION={}", redirect_response_server);

	tauri_build::build();
}
