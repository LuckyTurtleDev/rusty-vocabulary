use directories::ProjectDirs;
use iced::{
	button, text_input, widget::Space, Align, Button, Checkbox, Color, Column, Element, Length, Row, Sandbox, Settings,
	Text, TextInput,
};
use once_cell::sync::Lazy;
use std::{path::PathBuf, process::exit, string::String};

mod gui_errors;
use gui_errors::*;

mod config;
use config::*;

mod api;
use api::*;

mod activitys;
use activitys::main_menu::*;

const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
static PROJECT_DIRS: Lazy<ProjectDirs> =
	Lazy::new(|| ProjectDirs::from("de", "lukas1818", CARGO_PKG_NAME).expect_gui("failed to get project dirs"));
static CONFIG_FILE: Lazy<PathBuf> = Lazy::new(|| PROJECT_DIRS.config_dir().join("config.toml"));

#[derive(Debug, Clone)]
pub enum MsgLogin {
	TextInputServer(String),
	TextInputUsername(String),
	TextInputPassword(String),
	Login,
	Signin,
}

#[derive(Debug, Clone)]
pub enum MsgAddVocabulary {
	Back,
	TextInputTags(String),
	TextInputQuestion(String),
	TextInputAnswer(String),
	CheckboxBothSidesToogle,
	Add,
}

#[derive(Debug, Clone)]
pub enum Message {
	Login(MsgLogin),
	MainMenu(MsgMainMenu),
	AddVocabulary(MsgAddVocabulary),
}

struct WinLogin {
	text_input_server: text_input::State,
	text_input_server_value: String,
	text_input_username: text_input::State,
	text_input_username_value: String,
	text_input_password: text_input::State,
	text_input_password_value: String,
	button_login: button::State,
	error: Option<String>,
}

struct WinAddVocabulary {
	button_back: button::State,
	text_input_tags: text_input::State,
	text_input_tags_value: String,
	checkbox_both_sides: bool,
	button_add: button::State,
	text_input_question: text_input::State,
	text_input_question_value: String,
	text_input_answer: text_input::State,
	text_input_answer_value: String,
}

#[derive(PartialEq)]
pub enum Activity {
	MainMenu,
	Query,
	AddVocabulary,
	Login,
}

pub struct Window {
	config: Config,
	activity: Activity,
	login: WinLogin,
	main_menu: WinMainMenu,
	add_vocabulary: WinAddVocabulary,
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
			config,
			activity,
			login: WinLogin {
				text_input_server: text_input::State::new(),
				text_input_server_value: String::new(),
				text_input_username: text_input::State::new(),
				text_input_username_value: String::new(),
				text_input_password: text_input::State::new(),
				text_input_password_value: String::new(),
				button_login: button::State::new(),
				error: None,
			},
			main_menu: activitys::main_menu::new(status),
			add_vocabulary: WinAddVocabulary {
				button_back: button::State::new(),
				text_input_tags: text_input::State::new(),
				text_input_tags_value: String::new(),
				checkbox_both_sides: true,
				button_add: button::State::new(),
				text_input_question: text_input::State::new(),
				text_input_question_value: String::new(),
				text_input_answer: text_input::State::new(),
				text_input_answer_value: String::new(),
			},
		}
	}

	fn title(&self) -> String {
		String::from("rusty-vocabulary")
	}

	fn update(&mut self, message: Self::Message) {
		match message {
			Message::Login(msg) => match msg {
				MsgLogin::TextInputServer(value) => self.login.text_input_server_value = value,
				MsgLogin::TextInputUsername(value) => self.login.text_input_username_value = value,
				MsgLogin::TextInputPassword(value) => self.login.text_input_password_value = value,
				MsgLogin::Login => {
					let answer = login(
						&self.login.text_input_server_value,
						&self.login.text_input_username_value,
						&self.login.text_input_password_value,
					);
					match answer {
						Err(error) => {
							eprintln!("Login Error: {error:?}");
							self.login.error = Some(format!("{error:?}"));
						},
						Ok(token) => {
							self.login.error = None;
							self.config.account = Some(Account {
								token,
								server: self.login.text_input_server_value.clone(),
							});
							save_config(&self.config);
							self.activity = Activity::MainMenu;
						},
					}
				},
				MsgLogin::Signin => unimplemented(),
			},
			Message::MainMenu(msg) => activitys::main_menu::update(self, msg),
			Message::AddVocabulary(msg) => match msg {
				MsgAddVocabulary::Back => self.activity = Activity::MainMenu,
				MsgAddVocabulary::TextInputTags(value) => self.add_vocabulary.text_input_tags_value = value,
				MsgAddVocabulary::CheckboxBothSidesToogle => {
					self.add_vocabulary.checkbox_both_sides = !self.add_vocabulary.checkbox_both_sides
				},
				MsgAddVocabulary::Add => unimplemented(),
				MsgAddVocabulary::TextInputQuestion(value) => self.add_vocabulary.text_input_question_value = value,
				MsgAddVocabulary::TextInputAnswer(value) => self.add_vocabulary.text_input_answer_value = value,
			},
		};
		if self.activity == Activity::MainMenu {
			let answer =
				get_status(self.config.account.as_ref().unwrap()).login_for_auth_error_else_panic(&mut self.activity, None);
			if let Some(value) = answer {
				self.main_menu.status = value
			};
		}
	}

	fn view(&mut self) -> Element<Self::Message> {
		match self.activity {
			Activity::Login => Row::new()
				.push(Space::with_width(Length::Fill))
				.push(
					Column::new()
						.push(Space::with_height(Length::Fill))
						.spacing(20)
						.align_items(Align::Center)
						.push(Text::new("Login"))
						.push({
							match self.login.error.clone() {
								Some(error) => Text::new(error).color(Color::from_rgb8(255, 0, 0)),
								None => Text::new("").height(Length::Units(0)), //dummy text
							}
						})
						.push(
							Row::new()
								.push(
									Column::new()
										.spacing(5)
										.push(Text::new("Server: "))
										.push(Text::new("Username: "))
										.push(Text::new("Password: ")),
								)
								.push(
									Column::new()
										.width(Length::Units(500))
										.spacing(5)
										.push(TextInput::new(
											&mut self.login.text_input_server,
											"https://rust-vocabulary.example.com",
											&self.login.text_input_server_value,
											|value| Message::Login(MsgLogin::TextInputServer(value)),
										))
										.push(TextInput::new(
											&mut self.login.text_input_username,
											"",
											&self.login.text_input_username_value,
											|value| Message::Login(MsgLogin::TextInputUsername(value)),
										))
										.push(
											TextInput::new(
												&mut self.login.text_input_password,
												"",
												&self.login.text_input_password_value,
												|value| Message::Login(MsgLogin::TextInputPassword(value)),
											)
											.password(),
										),
								),
						)
						.push(Row::new().push({
							let button = Button::new(&mut self.login.button_login, Text::new("Login"));
							if self.login.text_input_server_value != ""
								&& self.login.text_input_username_value != ""
								&& self.login.text_input_password_value != ""
							{
								button.on_press(Message::Login(MsgLogin::Login))
							} else {
								button
							}
						}))
						.push(Space::with_height(Length::Fill)),
				)
				.push(Space::with_width(Length::Fill))
				.into(),
			Activity::MainMenu => activitys::main_menu::view(self),
			Activity::Query => Text::new("TODO").into(),
			Activity::AddVocabulary => Column::new()
				.push(
					Row::new()
						.padding(5)
						.align_items(Align::Center)
						.push(Space::new(Length::Fill, Length::Shrink))
						.push(
							Button::new(&mut self.add_vocabulary.button_back, Text::new("back"))
								.on_press(Message::AddVocabulary(MsgAddVocabulary::Back)),
						)
						.push(Space::new(Length::Fill, Length::Shrink))
						.push(Text::new("subject/language"))
						.push(Space::new(Length::Fill, Length::Shrink))
						.push(Text::new("tags: "))
						.push(TextInput::new(
							&mut self.add_vocabulary.text_input_tags,
							" none",
							&self.add_vocabulary.text_input_tags_value,
							|value| Message::AddVocabulary(MsgAddVocabulary::TextInputTags(value)),
						))
						.push(Space::new(Length::Fill, Length::Shrink))
						.push(Checkbox::new(self.add_vocabulary.checkbox_both_sides, "bot sides", |_| {
							Message::AddVocabulary(MsgAddVocabulary::CheckboxBothSidesToogle)
						}))
						.push(Space::new(Length::Fill, Length::Shrink))
						.push(
							Button::new(&mut self.add_vocabulary.button_add, Text::new("add vocabulary"))
								.on_press(Message::AddVocabulary(MsgAddVocabulary::Add)),
						)
						.push(Space::new(Length::Fill, Length::Shrink)),
				)
				.push(
					Row::new()
						.push(TextInput::new(
							&mut self.add_vocabulary.text_input_question,
							"question",
							&self.add_vocabulary.text_input_question_value,
							|value| Message::AddVocabulary(MsgAddVocabulary::TextInputQuestion(value)),
						))
						.push(TextInput::new(
							&mut self.add_vocabulary.text_input_answer,
							"answer",
							&self.add_vocabulary.text_input_answer_value,
							|value| Message::AddVocabulary(MsgAddVocabulary::TextInputAnswer(value)),
						)),
				)
				.push(Space::new(Length::Shrink, Length::Units(5)))
				.into(),
		}
	}
}

fn main() -> iced::Result {
	Window::run(Settings::default())
}
