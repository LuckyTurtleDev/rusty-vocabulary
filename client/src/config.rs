use serde::Deserialize;

#[derive(Default, Deserialize)]
pub struct Config {
	pub token: Option<String>,
}
