use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
	pub token: Option<String>,
}
