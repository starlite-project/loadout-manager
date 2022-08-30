use std::{collections::HashMap, fs::OpenOptions, io::prelude::*, path::PathBuf};

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

	pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
		plugin::Builder::new("storage")
			.setup(move |app_handle| {
				let store_path =
					get_store_path(app_handle).expect("failed to find application directory");

				if !store_path.try_exists()? {
					let mut f = OpenOptions::new()
						.create_new(true)
						.write(true)
						.open(store_path.as_path())?;

					let default_data = serde_json::to_vec(&HashMap::<String, JsonValue>::new())?;

					f.write(&default_data)?;
					f.flush()?;
				}

				let mut file = OpenOptions::new().read(true).open(store_path.as_path())?;

				let store = {
					let mut data = Vec::new();

					file.read_to_end(&mut data)?;

					let cache = serde_json::from_slice(&data)?;

					Store { cache }
				};

				app_handle.manage(store);

				Ok(())
			})
			.on_event(|app_handle, event| {
				if let RunEvent::Exit = event {
					let store = app_handle.state::<Store>();

					if let Err(e) = store.save(app_handle) {
						eprintln!(
							"failed to save store {:?} with error {:?}",
							get_store_path(&app_handle),
							e
						);
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
	cache: HashMap<String, JsonValue>,
}

impl Store {
	fn save<R: Runtime>(&self, app: &AppHandle<R>) -> Result<()> {
		let store_path = get_store_path(&app).expect("failed to resolve application directory");

		let bytes = serde_json::to_vec(&self.cache)?;

		let mut f = OpenOptions::new().write(true).open(store_path)?;

		f.write_all(&bytes)?;
		f.flush()?;

		Ok(())
	}
}

fn get_store_path<R: Runtime>(handle: &AppHandle<R>) -> Option<PathBuf> {
	Some(handle.path_resolver().app_dir()?.join("data.json"))
}
