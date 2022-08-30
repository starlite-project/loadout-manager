use std::{
	collections::HashMap,
	fs::OpenOptions,
	io::{prelude::*, ErrorKind},
	path::PathBuf,
};

use serde_json::Value as JsonValue;
use tauri::{
	plugin::{self, TauriPlugin},
	AppHandle, Manager, RunEvent, Runtime,
};

use crate::Result;

pub struct StoreBuilder {
	defaults: Option<HashMap<String, JsonValue>>,
}

impl StoreBuilder {
	pub fn new() -> Self {
		Self { defaults: None }
	}

	pub fn defaults(mut self, defaults: HashMap<String, JsonValue>) -> Self {
		self.defaults = Some(defaults);
		self
	}

	pub fn default(mut self, key: String, value: JsonValue) -> Self {
		self.defaults
			// use with capacity for micro opt, we know we're inserting something immediately
			.get_or_insert(HashMap::with_capacity(1))
			.insert(key, value);

		self
	}

	pub fn build<R: Runtime>(self, path: PathBuf) -> TauriPlugin<R> {
		plugin::Builder::new("storage")
			.setup(move |app_handle| {
				let store_path = app_handle
					.path_resolver()
					.app_dir()
					.expect("failed to find application directory")
					.join(path);

				let mut created_file = false;

				if let Err(e) = OpenOptions::new()
					.create_new(true)
					.write(true)
					.open(store_path.as_path())
				{
					// if it already exists, do nothing, otherwise exit
					if e.kind() == ErrorKind::AlreadyExists {
						// noop
					} else {
						app_handle.exit(1);
					}
				} else {
					created_file = true
				}

				let mut open_opts = OpenOptions::new();

				open_opts.read(true);

				if created_file {
					open_opts.write(true);
				}

				let mut file = open_opts.open(store_path.as_path())?;

				let store = if created_file {
					let defaults = self.defaults.unwrap_or_default();

					let serialized = serde_json::to_vec(&defaults)?;

					file.write_all(&serialized)?;
					file.flush()?;

					Store {
						path: store_path,
						cache: HashMap::new(),
					}
				} else {
					let mut data = Vec::new();

					file.read_to_end(&mut data)?;

					let cached = serde_json::from_slice(&data)?;

					Store {
						path: store_path,
						cache: cached,
					}
				};

				app_handle.manage(store);

				Ok(())
			})
			.on_event(|app_handle, event| {
				if let RunEvent::Exit = event {
					let store = app_handle.state::<Store>();

					if let Err(e) = store.save(app_handle) {
						eprintln!("failed to save store {:?} with error {:?}", store.path, e);
					}
				}
			})
			.build()
	}
}

impl Default for StoreBuilder {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Clone)]
pub struct Store {
	path: PathBuf,
	cache: HashMap<String, JsonValue>,
}

impl Store {
	fn save<R: Runtime>(&self, app: &AppHandle<R>) -> Result<()> {
		let store_path = app
			.path_resolver()
			.app_dir()
			.expect("failed to resolve application directory")
			.join(self.path.as_path());

		let bytes = serde_json::to_vec(&self.cache)?;

		let mut f = OpenOptions::new().write(true).open(store_path)?;

		f.write_all(&bytes)?;
		f.flush()?;

		Ok(())
	}
}
