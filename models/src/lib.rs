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

#[derive(Deserialize, Serialize)]
pub struct Token {
	pub user_name: String,
	pub date: u64,
	pub server_version: String,
}
