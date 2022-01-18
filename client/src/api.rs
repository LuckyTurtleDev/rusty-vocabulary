use crate::gui_errors::*;
use rusty_vocabulary_models::*;

pub fn login(server: &str, username: &str, password: &str) {
	let answer = attohttpc::post(format!("{server}/login"))
		.json(&Login {
			username: username.into(),
			password: password.into(),
		})
		.unwrap()
		.send()
		.expect_gui_exit("login failed", 1);
}
