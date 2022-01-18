use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Login {
	pub username: String,
	pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct Info {
	pub about: String,
	pub version: String,
}
