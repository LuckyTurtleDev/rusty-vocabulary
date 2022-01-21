use anyhow::Result;
use attohttpc::{Method, RequestBuilder};
use rusty_vocabulary_models::*;

pub fn login(server: &str, username: &str, password: &str) -> Result<Token> {
	let answer: Token = RequestBuilder::try_new(Method::POST, format!("{server}/login"))?
		.json(&Login {
			username: username.into(),
			password: password.into(),
		})
		.unwrap()
		.send()?
		.json()?;
	Ok(answer)
}
