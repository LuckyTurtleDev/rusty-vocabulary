use config::ThemeTitel;
use directories::ProjectDirs;
use iced::{Element, Sandbox, Settings, Text};
use once_cell::sync::Lazy;
use std::{path::PathBuf, string::String};
use typetest_themes::ApplicationTheme;

mod gui_errors;
use gui_errors::*;

mod config;
use config::*;

mod api;
use api::*;

mod activitys;
use activitys::{add::*, login::*, main_menu::*, settings::*, Activity, Message};

const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
static PROJECT_DIRS: Lazy<ProjectDirs> =
	Lazy::new(|| ProjectDirs::from("de", "lukas1818", CARGO_PKG_NAME).expect_gui("failed to get project dirs"));
static CONFIG_FILE: Lazy<PathBuf> = Lazy::new(|| PROJECT_DIRS.config_dir().join("config.toml"));

pub struct Window {
	theme: Box<dyn ApplicationTheme>,
	config: Config,
	activity: Activity,
	login: WinLogin,
	main_menu: WinMainMenu,
	add: WinAdd,
	settings: WinSettings,
}

impl Sandbox for Window {
	type Message = Message;

	fn new() -> Window {
		let config = load_config();
		let mut activity = Activity::MainMenu;
		let mut status = rusty_vocabulary_models::Status::default();
		if config.account.is_none() {
			activity = Activity::Login
		} else {
			let answer = get_status(config.account.as_ref().unwrap()).login_for_auth_error_else_panic(&mut activity, None);
			if let Some(value) = answer {
				status = value
			};
		}
		Window {
			theme: match config.theme.unwrap_or_default() {
				ThemeTitel::Dark => typetest_themes::Theme::DefaultDark.into(),
				ThemeTitel::Light => typetest_themes::Theme::DefaultLight.into(),
			},
			config,
			activity,
			login: activitys::login::new(),
			main_menu: activitys::main_menu::new(status),
			add: activitys::add::new(),
			settings: activitys::settings::new(),
		}
	}

	fn title(&self) -> String {
		String::from("rusty-vocabulary")
	}

	fn update(&mut self, message: Self::Message) {
		match message {
			Message::Login(msg) => activitys::login::update(self, msg),
			Message::MainMenu(msg) => activitys::main_menu::update(self, msg),
			Message::Add(msg) => activitys::add::update(self, msg),
			Message::Settings(msg) => activitys::settings::update(self, msg),
		};
		if self.activity == Activity::MainMenu {
			activitys::main_menu::post_update(self);
		}
	}

	fn view(&mut self) -> Element<Self::Message> {
		match self.activity {
			Activity::Login => activitys::login::view(self),
			Activity::MainMenu => activitys::main_menu::view(self),
			Activity::Query => Text::new("TODO").into(),
			Activity::Settings => activitys::settings::view(self),
			Activity::Add => activitys::add::view(self),
		}
	}
}

fn main() -> iced::Result {
	Window::run(Settings::default())
}
