use iced::{button, text_input, widget::Space, Align, Button, Color, Column, Element, Length, Row, Text, TextInput};

use std::string::String;

use crate::{api::*, config::*, gui_errors::*};

use super::{Activity, Message};

#[derive(Debug, Clone)]
pub enum MsgLogin {
	TextInputServer(String),
	TextInputUsername(String),
	TextInputPassword(String),
	Login,
	Signin,
}

pub struct WinLogin {
	pub text_input_server: text_input::State,
	pub text_input_server_value: String,
	pub text_input_username: text_input::State,
	pub text_input_username_value: String,
	pub text_input_password: text_input::State,
	pub text_input_password_value: String,
	pub button_login: button::State,
	pub error: Option<String>,
}

pub fn new() -> WinLogin {
	WinLogin {
		text_input_server: text_input::State::new(),
		text_input_server_value: String::new(),
		text_input_username: text_input::State::new(),
		text_input_username_value: String::new(),
		text_input_password: text_input::State::new(),
		text_input_password_value: String::new(),
		button_login: button::State::new(),
		error: None,
	}
}

pub fn update(win: &mut super::Window, message: MsgLogin) {
	match message {
		MsgLogin::TextInputServer(value) => win.login.text_input_server_value = value,
		MsgLogin::TextInputUsername(value) => win.login.text_input_username_value = value,
		MsgLogin::TextInputPassword(value) => win.login.text_input_password_value = value,
		MsgLogin::Login => {
			let answer = login(
				&win.login.text_input_server_value,
				&win.login.text_input_username_value,
				&win.login.text_input_password_value,
			);
			match answer {
				Err(error) => {
					eprintln!("Login Error: {error:?}");
					win.login.error = Some(format!("{error:?}"));
				},
				Ok(token) => {
					win.login.error = None;
					win.config.account = Some(Account {
						token,
						server: win.login.text_input_server_value.clone(),
					});
					save_config(&win.config);
					win.activity = Activity::MainMenu;
				},
			}
		},
		MsgLogin::Signin => unimplemented(),
	}
}

pub fn view(win: &mut super::Window) -> Element<super::Message> {
	Row::new()
		.push(Space::with_width(Length::Fill))
		.push(
			Column::new()
				.push(Space::with_height(Length::Fill))
				.spacing(20)
				.align_items(Align::Center)
				.push(Text::new("Login"))
				.push({
					match win.login.error.clone() {
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
									&mut win.login.text_input_server,
									"https://rust-vocabulary.example.com",
									&win.login.text_input_server_value,
									|value| Message::Login(MsgLogin::TextInputServer(value)),
								))
								.push(TextInput::new(
									&mut win.login.text_input_username,
									"",
									&win.login.text_input_username_value,
									|value| Message::Login(MsgLogin::TextInputUsername(value)),
								))
								.push(
									TextInput::new(
										&mut win.login.text_input_password,
										"",
										&win.login.text_input_password_value,
										|value| Message::Login(MsgLogin::TextInputPassword(value)),
									)
									.password(),
								),
						),
				)
				.push(Row::new().push({
					let button = Button::new(&mut win.login.button_login, Text::new("Login"));
					if win.login.text_input_server_value != ""
						&& win.login.text_input_username_value != ""
						&& win.login.text_input_password_value != ""
					{
						button.on_press(Message::Login(MsgLogin::Login))
					} else {
						button
					}
				}))
				.push(Space::with_height(Length::Fill)),
		)
		.push(Space::with_width(Length::Fill))
		.into()
}
