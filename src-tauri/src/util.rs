use std::{
	cmp::Ordering,
	convert::Infallible,
	error::Error,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
	hash::{Hash, Hasher},
};

use serde::{Deserialize, Serialize};

pub const API_BASE: &str = "https://www.bungie.net/";

pub const API_KEY: &str = env!("API_KEY");

pub const CLIENT_ID: &str = env!("CLIENT_ID");

pub const CLIENT_SECRET: &str = env!("CLIENT_SECRET");

pub mod values_as_strings {
	use std::{fmt::Display, str::FromStr};

	use serde::{Deserialize, Deserializer, Serializer};

	pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
	where
		D: Deserializer<'de>,
		T: FromStr,
		<T as FromStr>::Err: Display,
	{
		let s = String::deserialize(deserializer)?;
		s.parse().map_err(serde::de::Error::custom)
	}

	pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
		T: ToString,
	{
		let s = value.to_string();
		serializer.serialize_str(&s)
	}
}

#[derive(Copy, Serialize, Deserialize)]
pub enum Impossible {}

impl Clone for Impossible {
	fn clone(&self) -> Self {
		match *self {}
	}
}

impl Debug for Impossible {
	fn fmt(&self, _: &mut Formatter<'_>) -> FmtResult {
		match *self {}
	}
}

impl Display for Impossible {
	fn fmt(&self, _: &mut Formatter<'_>) -> FmtResult {
		match *self {}
	}
}

impl PartialEq for Impossible {
	fn eq(&self, _: &Self) -> bool {
		match *self {}
	}
}

impl Eq for Impossible {}

impl PartialOrd for Impossible {
	fn partial_cmp(&self, _: &Self) -> Option<Ordering> {
		match *self {}
	}
}

impl Ord for Impossible {
	fn cmp(&self, _: &Self) -> Ordering {
		match *self {}
	}
}

impl From<Infallible> for Impossible {
	fn from(x: Infallible) -> Self {
		match x {}
	}
}

impl Hash for Impossible {
	fn hash<H: Hasher>(&self, _: &mut H) {
		match *self {}
	}
}

impl Error for Impossible {}
