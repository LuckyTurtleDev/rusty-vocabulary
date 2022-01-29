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

#[derive(Clone, Deserialize, Serialize)]
pub struct Token {
	pub user_name: String,
	pub iat: u64,
	pub exp: u64,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Status {
	pub vocabulary: u64,
	pub outstanding_vocabulary: u64,
	pub subjects: u16,
	pub outstanding_subjects: u16,
}
