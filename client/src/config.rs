use crate::{api::Account, gui_errors::*, CONFIG_FILE};
use serde::{Deserialize, Serialize};
use std::fs;
use strum_macros::Display;

pub fn load_config() -> Config {
	let file_content = fs::read_to_string(CONFIG_FILE.as_path());
	match file_content {
		Ok(file_content) => toml::from_str(&file_content)
			.expect_gui_exit(&format!("failed to parse config file \"{}\"", CONFIG_FILE.display()), 1),
		Err(error) => {
			if error.kind() != std::io::ErrorKind::NotFound {
				gui_exit_with_error(
					&format!("failed to open config file \"{}\":\n{error}", CONFIG_FILE.display()),
					1,
				);
			}
			eprintln!("config file \"{}\" not found, use default values", CONFIG_FILE.display());
			Config::default()
		},
	}
}

pub fn save_config(config: &Config) {
	fs::create_dir_all(CONFIG_FILE.as_path().parent().unwrap()).expect_gui_exit(
		&format!("can not creade folder {:?}", CONFIG_FILE.as_path().parent().unwrap()),
		1,
	);
	let result = fs::write(CONFIG_FILE.as_path(), toml::to_string_pretty(config).unwrap());
	if result.is_err() {
		gui_exit_with_error(
			&format!(
				"failed to save config file \"{}\":\n{}",
				CONFIG_FILE.display(),
				result.unwrap_err()
			),
			1,
		);
	}
}

#[derive(Clone, Copy, Debug, Deserialize, Display, PartialEq, Eq, Serialize)]
#[strum(serialize_all = "lowercase")]
pub enum ThemeTitel {
	Light,
	Dark,
}

impl Default for ThemeTitel {
	fn default() -> ThemeTitel {
		ThemeTitel::Dark
	}
}

#[derive(Default, Deserialize, Serialize)]
pub struct Config {
	pub account: Option<Account>,
	pub theme: Option<ThemeTitel>,
}
