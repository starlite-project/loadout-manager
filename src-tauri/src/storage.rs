use std::{borrow::Cow, collections::HashMap, fs::OpenOptions, io::prelude::*, path::PathBuf};

use serde::Serialize;
use serde_json::Value as JsonValue;
use tauri::{
	async_runtime::RwLock,
	plugin::{self, TauriPlugin},
	AppHandle, Manager, RunEvent, Runtime, State, Window,
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
			.invoke_handler(tauri::generate_handler![
				set, get, has, delete, clear, keys, values, entries, length
			])
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

					Store {
						cache: RwLock::new(cache),
					}
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

#[derive(Debug)]
pub struct Store {
	cache: RwLock<HashMap<String, JsonValue>>,
}

impl Store {
	fn save<R: Runtime>(&self, app: &AppHandle<R>) -> Result<()> {
		let store_path = get_store_path(&app).expect("failed to resolve application directory");

		let cache = self.cache.blocking_read();

		let bytes = serde_json::to_vec(&*cache)?;

		let mut f = OpenOptions::new().write(true).open(store_path)?;

		f.write_all(&bytes)?;
		f.flush()?;

		Ok(())
	}
}

fn get_store_path<R: Runtime>(handle: &AppHandle<R>) -> Option<PathBuf> {
	Some(handle.path_resolver().app_dir()?.join("data.json"))
}

#[derive(Clone, Serialize)]
struct ChangePayload<'a> {
	key: Cow<'a, str>,
	value: JsonValue,
}

// these have to return a result as trying to just return causes lifetime errors withing the generate_handler macro
#[tauri::command]
async fn set<R: Runtime>(
	window: Window<R>,
	store: State<'_, Store>,
	key: String,
	value: JsonValue,
) -> Result<()> {
	let mut cache = store.cache.write().await;
	cache.insert(key.clone(), value.clone());
	let _ = window.emit(
		"store://change",
		ChangePayload {
			key: Cow::Owned(key),
			value,
		},
	);
	Ok(())
}

#[tauri::command]
async fn get(store: State<'_, Store>, key: &str) -> Result<Option<JsonValue>> {
	let cache = store.cache.read().await;
	Ok(cache.get(key).cloned())
}

#[tauri::command]
async fn has(store: State<'_, Store>, key: &str) -> Result<bool> {
	let cache = store.cache.read().await;
	Ok(cache.contains_key(key))
}

#[tauri::command]
async fn delete<R: Runtime>(window: Window<R>, store: State<'_, Store>, key: &str) -> Result<bool> {
	let mut cache = store.cache.write().await;
	let flag = cache.remove(key).is_some();
	if flag {
		let _ = window.emit(
			"store://change",
			ChangePayload {
				key: Cow::Borrowed(key),
				value: JsonValue::Null,
			},
		);
	}

	Ok(flag)
}

#[tauri::command]
async fn clear<R: Runtime>(window: Window<R>, store: State<'_, Store>) -> Result<()> {
	let mut cache = store.cache.write().await;
	let keys = cache.keys().cloned().collect::<Vec<String>>();
	cache.clear();
	for key in keys {
		let _ = window.emit(
			"store://change",
			ChangePayload {
				key: Cow::Owned(key),
				value: JsonValue::Null,
			},
		);
	}

	Ok(())
}

#[tauri::command]
async fn keys(store: State<'_, Store>) -> Result<Vec<String>> {
	let cache = store.cache.read().await;
	Ok(cache.keys().cloned().collect())
}

#[tauri::command]
async fn values(store: State<'_, Store>) -> Result<Vec<JsonValue>> {
	let cache = store.cache.read().await;
	Ok(cache.values().cloned().collect())
}

#[tauri::command]
async fn entries(store: State<'_, Store>) -> Result<HashMap<String, JsonValue>> {
	let cache = store.cache.read().await;
	Ok(cache.clone())
}

#[tauri::command]
async fn length(store: State<'_, Store>) -> Result<usize> {
	let cache = store.cache.read().await;
	Ok(cache.len())
}
