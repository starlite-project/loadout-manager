use std::{
	error::Error as StdError,
	fmt::{Display, Formatter, Result as FmtResult},
	io::Error as IoError,
	path::PathBuf,
};

use serde::{Serialize, Serializer};
use serde_json::Error as JsonError;

#[derive(Debug)]
pub enum Error {
	Serialize(Box<dyn StdError>),
	Deserialize(Box<dyn StdError>),
	Json(JsonError),
	Io(IoError),
	NotFound(PathBuf),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Serialize(e) => {
				f.write_str("Failed to serialize store. ")?;
				Display::fmt(e, f)
			}
			Self::Deserialize(e) => {
				f.write_str("Failed to deserialize store. ")?;
				Display::fmt(e, f)
			}
			Self::Json(e) => Display::fmt(e, f),
			Self::Io(e) => Display::fmt(e, f),
			Self::NotFound(p) => {
				f.write_str("Store \"")?;
				Display::fmt(&p.display(), f)?;
				f.write_str("\" not found")
			}
		}
	}
}

impl StdError for Error {}

impl From<JsonError> for Error {
	fn from(e: JsonError) -> Self {
		Self::Json(e)
	}
}

impl From<IoError> for Error {
	fn from(e: IoError) -> Self {
		Self::Io(e)
	}
}

impl Serialize for Error {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(self.to_string().as_str())
	}
}
