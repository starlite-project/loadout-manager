use std::{
	collections::HashMap,
	error::Error as StdError,
	fmt::{Debug, Formatter, Result as FmtResult},
	fs::{create_dir_all, read, File},
	io::Write,
	path::PathBuf,
};

use serde_json::Value as JsonValue;
use tauri::{AppHandle, Runtime};

use crate::Error;

type SerializeFn = fn(&HashMap<String, JsonValue>) -> Result<Vec<u8>, Box<dyn StdError>>;
type DeserializeFn = fn(&[u8]) -> Result<HashMap<String, JsonValue>, Box<dyn StdError>>;

fn default_serialize(cache: &HashMap<String, JsonValue>) -> Result<Vec<u8>, Box<dyn StdError>> {
	Ok(serde_json::to_vec(&cache)?)
}

fn default_deserialize(bytes: &[u8]) -> Result<HashMap<String, JsonValue>, Box<dyn StdError>> {
	serde_json::from_slice(bytes).map_err(Into::into)
}

pub struct StoreBuilder {
	path: PathBuf,
	defaults: Option<HashMap<String, JsonValue>>,
	cache: HashMap<String, JsonValue>,
	serialize: SerializeFn,
	deserialize: DeserializeFn,
}

impl StoreBuilder {
	pub fn new(path: PathBuf) -> Self {
		Self {
			path,
			defaults: None,
			cache: HashMap::default(),
			serialize: default_serialize,
			deserialize: default_deserialize,
		}
	}

	pub fn defaults(mut self, defaults: HashMap<String, JsonValue>) -> Self {
		self.cache = defaults.clone();
		self.defaults = Some(defaults);
		self
	}

	pub fn serialize(mut self, serialize: SerializeFn) -> Self {
		self.serialize = serialize;
		self
	}

	pub fn deserialize(mut self, deserialize: DeserializeFn) -> Self {
		self.deserialize = deserialize;
		self
	}

	pub fn build(self) -> Store {
		Store {
			path: self.path,
			defaults: self.defaults,
			cache: self.cache,
			serialize: self.serialize,
			deserialize: self.deserialize,
		}
	}
}

impl Default for StoreBuilder {
	fn default() -> Self {
		Self::new(PathBuf::default())
	}
}

#[derive(Clone)]
pub struct Store {
	pub(crate) path: PathBuf,
	pub(crate) defaults: Option<HashMap<String, JsonValue>>,
	pub(crate) cache: HashMap<String, JsonValue>,
	serialize: SerializeFn,
	deserialize: DeserializeFn,
}

impl Store {
	pub fn load<R: Runtime>(&mut self, app: &AppHandle<R>) -> Result<(), Error> {
		let app_dir = app
			.path_resolver()
			.app_dir()
			.expect("failed to resolve app directory");
		let store_path = app_dir.join(&self.path);

		let bytes = read(&store_path)?;

		self.cache = (self.deserialize)(&bytes).map_err(Error::Deserialize)?;

		Ok(())
	}

	pub fn save<R: Runtime>(&self, app: &AppHandle<R>) -> Result<(), Error> {
		let app_dir = app
			.path_resolver()
			.app_dir()
			.expect("failed to resolve app directory");

		let store_path = app_dir.join(&self.path);

		create_dir_all(store_path.parent().expect("invalid store path"))?;

		let bytes = (self.serialize)(&self.cache).map_err(Error::Serialize)?;
		let mut f = File::create(&store_path)?;
		f.write_all(&bytes)?;

		Ok(())
	}
}

impl Debug for Store {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		f.debug_struct("Store")
			.field("path", &self.path)
			.field("defaults", &self.defaults)
			.field("cache", &self.cache)
			.finish()
	}
}
