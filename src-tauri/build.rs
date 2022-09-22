use std::{env, path::PathBuf};

const CURRENT_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
	let is_release = match env::var("PROFILE").unwrap().as_str() {
		"debug" => false,
		"release" => true,
		_ => panic!("unexpected value set for PROFILE env"),
	};

	let base_path = PathBuf::from(CURRENT_DIR);

	let full_path = if is_release {
		base_path.join(".env.production")
	} else {
		base_path.join(".env.development")
	};

	dotenv::from_path(full_path).expect("failed to read file");

	let res = ["api_key", "client_id", "client_secret"]
		.into_iter()
		.map(set_env_value)
		.collect::<Result<(), env::VarError>>();

	res.expect("failed to set a hardcoded value, maybe it's not set in the environment?");

	set_env_or_default("server_location", "https://starlight-loadout-server.com");

	tauri_build::build();
}

fn set_env_value(key: &str) -> Result<(), env::VarError> {
	let key = key.to_ascii_uppercase();
	let value = env::var(key.as_str())?;

	println!("cargo:rustc-env={}={}", key, value);
	Ok(())
}

fn set_env_or_default(key: &str, default_value: &str) {
	let key = key.to_ascii_uppercase();
	// call to_owned here to avoid an allocation if possible
	let value = env::var(key.as_str()).unwrap_or_else(|_| default_value.to_owned());
	println!("cargo:rustc-env={}={}", key, value);
}
