use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer};

pub fn deserialize_string_as<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: FromStr,
	<T as FromStr>::Err: Display,
{
	let s = String::deserialize(deserializer)?;
	s.parse().map_err(serde::de::Error::custom)
}
