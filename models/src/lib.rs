use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Login {
	pub username: String,
	pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Info {
	pub about: String,
	pub version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Token {
	pub user_name: String,
	pub iat: u64,
	pub exp: u64,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Status {
	pub vocabulary: u64,
	pub outstanding_vocabulary: u64,
	pub subjects: u16,
	pub outstanding_subjects: u16,
}

pub mod card {
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Deserialize, Serialize)]
	pub struct Content {
		pub question: String,
		pub answer: String,
	}

	#[derive(Debug, Deserialize, Serialize)]
	pub struct MetaData {
		pub subject: String,
		pub tags: Vec<String>,
	}

	#[derive(Debug, Deserialize, Serialize)]
	pub struct Rating {
		pub repetition: u16,
		pub easiness: f32,
		pub due_date: u64,
	}

	#[derive(Debug, Deserialize, Serialize)]
	pub struct Full {
		pub id: u64,
		pub content: Content,
		pub meta_data: MetaData,
		pub rating: Rating,
	}

	#[derive(Debug, Deserialize, Serialize)]
	pub struct Small {
		pub id: u64,
		pub content: Content,
	}

	#[derive(Debug, Deserialize, Serialize)]
	pub struct Medium {
		pub id: u64,
		pub content: Content,
		pub meta_data: MetaData,
	}

	#[derive(Debug, Deserialize, Serialize)]
	pub struct New {
		pub content: Content,
		pub meta_data: MetaData,
	}
}
