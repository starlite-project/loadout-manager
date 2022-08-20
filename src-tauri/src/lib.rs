use tauri::api::http::{Client, ClientBuilder};

pub mod fetch;
mod model;
pub mod util;

#[derive(Debug, Clone)]
pub struct LoadoutState {
	http_client: Client,
}

impl LoadoutState {
	pub fn new() -> Result<Self, tauri::Error> {
		let http_client = ClientBuilder::new().build()?;

		Ok(Self { http_client })
	}

	pub fn http(&self) -> &Client {
		&self.http_client
	}
}
