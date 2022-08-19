use std::env;

fn main() {
	dotenv::dotenv().unwrap();
	let api_key = env::var("API_KEY").unwrap();
	println!("cargo:rustc-env=API_KEY={}", api_key);
	tauri_build::build();
}
