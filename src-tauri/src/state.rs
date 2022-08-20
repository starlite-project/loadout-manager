use tauri::api::http::{Client, ClientBuilder};

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
