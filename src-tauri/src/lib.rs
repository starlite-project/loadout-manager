use std::{
	error::Error as StdError,
	fmt::{Display, Formatter, Result as FmtResult},
};

pub mod fetch;
mod model;
mod state;
pub mod util;

use serde::{Serialize, Serializer};

pub use self::state::LoadoutState;

#[derive(Debug)]
#[repr(transparent)]
pub struct Error(anyhow::Error);

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Display::fmt(&self.0, f)
	}
}

impl<E> From<E> for Error
where
	E: StdError + Send + Sync + 'static,
{
	fn from(e: E) -> Self {
		Self(e.into())
	}
}

impl From<Error> for anyhow::Error {
	fn from(e: Error) -> anyhow::Error {
		e.0
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

pub type Result<T, E = Error> = std::result::Result<T, E>;
