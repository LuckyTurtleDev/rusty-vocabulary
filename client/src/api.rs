use anyhow::{bail, Context};
use attohttpc::{Method, RequestBuilder};
use rusty_vocabulary_models::*;

pub fn check_server(server: &str) -> anyhow::Result<()> {
	let answer = RequestBuilder::try_new(Method::GET, format!("{server}/info"))?.send()?;
	let info: Result<Info, attohttpc::Error> = answer.json();
	if match info {
		Err(_) => true,
		Ok(info) => {
			if info.about != "rusty-vocabulary" {
				true
			} else {
				false
			}
		},
	} {
		bail!("Rusty-vocabulary is not running at this server");
	}
	Ok(())
}

pub fn login(server: &str, username: &str, password: &str) -> anyhow::Result<Token> {
	check_server(&server).with_context(|| format!("Failed to connect to Server"))?;
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
