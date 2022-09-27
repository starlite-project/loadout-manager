use std::{
	error::Error,
	fmt::{Display, Formatter, Result as FmtResult, Write},
};

use serde::Serialize;

#[derive(Debug)]
pub enum FetchError {
	Bungie {
		code: i32,
		message: String,
		detailed_error_trace: Option<String>,
	},
	Reqwest(reqwest::Error),
	Url(url::ParseError),
}

impl Display for FetchError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Self::Bungie {
				code,
				message,
				detailed_error_trace,
			} => {
				f.write_str("error code: ")?;
				Display::fmt(code, f)?;
				f.write_char('\n')?;
				f.write_str("message: ")?;
				f.write_str(message)?;
				if let Some(trace) = detailed_error_trace.as_deref() {
					f.write_char('\n')?;
					f.write_str("stack trace: ")?;
					f.write_str(trace)
				} else {
					Ok(())
				}
			}
			Self::Reqwest(e) => Display::fmt(e, f),
			Self::Url(e) => Display::fmt(e, f),
		}
	}
}

impl Error for FetchError {}

impl From<reqwest::Error> for FetchError {
	fn from(e: reqwest::Error) -> Self {
		Self::Reqwest(e)
	}
}

impl From<url::ParseError> for FetchError {
	fn from(e: url::ParseError) -> Self {
		Self::Url(e)
	}
}

impl Serialize for FetchError {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}
