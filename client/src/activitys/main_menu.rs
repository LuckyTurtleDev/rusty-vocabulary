use iced::{button, widget::Space, Button, Column, Element, Length, Row, Text};
use std::process::exit;

use rusty_vocabulary_models::Status;

use crate::api::*;

use super::{Activity, Message};

#[derive(Debug, Clone, Copy)]
pub enum MsgMainMenu {
	Query,
	Add,
	Quit,
}

pub struct WinMainMenu {
	pub status: rusty_vocabulary_models::Status,
	pub button_query: button::State,
	pub button_add: button::State,
	pub button_explore: button::State,
	pub button_quit: button::State,
}

pub fn new(status: Status) -> WinMainMenu {
	WinMainMenu {
		status,
		button_query: button::State::new(),
		button_add: button::State::new(),
		button_explore: button::State::new(),
		button_quit: button::State::new(),
	}
}

pub fn update(win: &mut super::Window, message: MsgMainMenu) {
	match message {
		MsgMainMenu::Quit => exit(0),
		MsgMainMenu::Add => win.activity = Activity::Add,
		MsgMainMenu::Query => win.activity = Activity::Query,
	}
}

pub fn post_update(win: &mut super::Window) {
	let answer = get_status(win.config.account.as_ref().unwrap()).login_for_auth_error_else_panic(&mut win.activity, None);
	if let Some(value) = answer {
		win.main_menu.status = value
	};
}

pub fn view(win: &mut super::Window) -> Element<super::Message> {
	Row::new()
		.push(Space::new(Length::Fill, Length::Shrink))
		.push(
			Column::new()
			.push(Space::new(Length::Shrink, Length::Fill))
			.push(Text::new(format!("total vocabulary: {}", win.main_menu.status.vocabulary))) //todo: use //align_items(Alignment::Fill)
			.push(Text::new(format!("outstanding vocabulary: {}", win.main_menu.status.outstanding_vocabulary)))
			.push(Text::new(format!("subjects/languages: {}", win.main_menu.status.subjects)))
			.push(Space::new(Length::Shrink, Length::Fill)),
		)
		.push(Space::new(iced::Length::Units(20), Length::Shrink))
		.push(
			Column::new()
				.push(Space::new(Length::Shrink, Length::Fill))
				.push(
					Button::new(&mut win.main_menu.button_query, Text::new("query vocabulary"))
						.on_press(Message::MainMenu(MsgMainMenu::Query)),
				)
				.push(
					Button::new(&mut win.main_menu.button_add, Text::new("add vocabulary"))
						.on_press(Message::MainMenu(MsgMainMenu::Add)),
				)
				.push(Button::new(
					&mut win.main_menu.button_explore,
					Text::new("explore vocabulary"),
				))
				.push(
					Button::new(&mut win.main_menu.button_quit, Text::new("quit"))
						.on_press(Message::MainMenu(MsgMainMenu::Quit)),
				)
				.push(Space::new(Length::Shrink, Length::Fill)),
		)
		.push(Space::new(Length::Fill, Length::Shrink))
		.into()
}
