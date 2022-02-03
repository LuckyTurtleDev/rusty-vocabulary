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

pub mod card {
	use serde::{Deserialize, Serialize};

	#[derive(Deserialize, Serialize)]
	pub struct Content {
		pub question: String,
		pub answer: String,
	}

	#[derive(Deserialize, Serialize)]
	pub struct MetaData {
		pub subject: String,
		pub tags: Vec<String>,
	}

	#[derive(Deserialize, Serialize)]
	pub struct Rating {
		pub repetition: u16,
		pub easiness: f32,
		pub due_date: u64,
	}

	#[derive(Deserialize, Serialize)]
	pub struct Full {
		pub id: u64,
		pub content: Content,
		pub meta_data: MetaData,
		pub rating: Rating,
	}

	#[derive(Deserialize, Serialize)]
	pub struct Small {
		pub id: u64,
		pub content: Content,
	}

	#[derive(Deserialize, Serialize)]
	pub struct Medium {
		pub id: u64,
		pub content: Content,
		pub meta_data: MetaData,
	}

	#[derive(Deserialize, Serialize)]
	pub struct New {
		pub content: Content,
		pub meta_data: MetaData,
	}
}
