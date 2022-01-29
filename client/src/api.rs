use crate::{gui_panic, Activity};
use anyhow::{bail, Context};
use attohttpc::{Method, RequestBuilder};
use rusty_vocabulary_models::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Account {
	pub token: String,
	pub server: String,
}

pub fn check_server(server: &str) -> anyhow::Result<()> {
	let answer = RequestBuilder::try_new(Method::GET, format!("{server}/info"))?
		.send()?
		.error_for_status()?;
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

pub fn login(server: &str, username: &str, password: &str) -> anyhow::Result<String> {
	check_server(&server).with_context(|| format!("Failed to connect to Server"))?;
	let answer: String = RequestBuilder::try_new(Method::POST, format!("{server}/login"))?
		.json(&Login {
			username: username.into(),
			password: password.into(),
		})
		.unwrap()
		.send()?
		.error_for_status()?
		.text()?;
	Ok(answer)
}

pub fn get_status(account: &Account) -> attohttpc::Result<Status> {
	attohttpc::get(format!("{}/status", account.server))
		.bearer_auth(&account.token)
		.send()?
		.error_for_status()?
		.json()
}

pub trait LoginForAuthError<T> {
	fn login_for_auth_error_else_panic(self, activity: &mut crate::Activity, msg: Option<&str>) -> Option<T>;
}

impl<T> LoginForAuthError<T> for attohttpc::Result<T> {
	fn login_for_auth_error_else_panic(self, activity: &mut crate::Activity, msg: Option<&str>) -> Option<T> {
		match self {
			Ok(value) => Some(value),
			Err(error) => match error.kind() {
				attohttpc::ErrorKind::StatusCode(code) => {
					if *code == 403 {
						eprintln!("Request failed: {error:?}");
						*activity = Activity::Login
					} else {
						gui_panic(&format!("{} {:?}", msg.unwrap_or("request failed:"), error));
					};
					None
				},
				_ => gui_panic(&format!("{} {:?}", msg.unwrap_or("request failed:"), error)),
			},
		}
	}
}
