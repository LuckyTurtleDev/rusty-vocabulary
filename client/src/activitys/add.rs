use iced::{button, text_input, widget::Space, Align, Button, Checkbox, Column, Element, Length, Row, Text, TextInput};

use std::string::String;

use crate::api::{add_card, LoginForAuthError};
use rusty_vocabulary_models::card;

use super::{Activity, Message};

#[derive(Debug, Clone)]
pub enum MsgAdd {
	Back,
	TextInputTags(String),
	TextInputQuestion(String),
	TextInputAnswer(String),
	CheckboxBothSidesToogle,
	Add,
}

pub struct WinAdd {
	pub button_back: button::State,
	pub text_input_tags: text_input::State,
	pub text_input_tags_value: String,
	pub checkbox_both_sides: bool,
	pub button_add: button::State,
	pub text_input_question: text_input::State,
	pub text_input_question_value: String,
	pub text_input_answer: text_input::State,
	pub text_input_answer_value: String,
}

pub fn new() -> WinAdd {
	WinAdd {
		button_back: button::State::new(),
		text_input_tags: text_input::State::new(),
		text_input_tags_value: String::new(),
		checkbox_both_sides: true,
		button_add: button::State::new(),
		text_input_question: text_input::State::new(),
		text_input_question_value: String::new(),
		text_input_answer: text_input::State::new(),
		text_input_answer_value: String::new(),
	}
}

pub fn update(win: &mut super::Window, message: MsgAdd) {
	match message {
		MsgAdd::Back => win.activity = Activity::MainMenu,
		MsgAdd::TextInputTags(value) => win.add.text_input_tags_value = value,
		MsgAdd::CheckboxBothSidesToogle => win.add.checkbox_both_sides = !win.add.checkbox_both_sides,
		MsgAdd::Add => {
			let card = card::New {
				content: card::Content {
					question: win.add.text_input_question_value.clone(),
					answer: win.add.text_input_answer_value.clone(),
				},
				meta_data: card::MetaData {
					subject: "todo".into(),
					tags: win.add.text_input_tags_value.split(' ').map(|s| s.to_string()).collect(),
				},
			};
			let answer = add_card(win.config.account.as_ref().unwrap(), card)
				.login_for_auth_error_else_panic(&mut win.activity, None);
			if answer.is_some() {
				win.add.text_input_answer_value = "".into();
				win.add.text_input_question_value = "".into();
			}
		},
		MsgAdd::TextInputQuestion(value) => win.add.text_input_question_value = value,
		MsgAdd::TextInputAnswer(value) => win.add.text_input_answer_value = value,
	}
}

pub fn view(win: &mut super::Window) -> Element<super::Message> {
	Column::new()
		.push(
			Row::new()
				.padding(5)
				.align_items(Align::Center)
				.push(Space::new(Length::Fill, Length::Shrink))
				.push(Button::new(&mut win.add.button_back, Text::new("back")).on_press(Message::Add(MsgAdd::Back)))
				.push(Space::new(Length::Fill, Length::Shrink))
				.push(Text::new("subject/language"))
				.push(Space::new(Length::Fill, Length::Shrink))
				.push(Text::new("tags: "))
				.push(TextInput::new(
					&mut win.add.text_input_tags,
					" none",
					&win.add.text_input_tags_value,
					|value| Message::Add(MsgAdd::TextInputTags(value)),
				))
				.push(Space::new(Length::Fill, Length::Shrink))
				.push(Checkbox::new(win.add.checkbox_both_sides, "bot sides", |_| {
					Message::Add(MsgAdd::CheckboxBothSidesToogle)
				}))
				.push(Space::new(Length::Fill, Length::Shrink))
				.push({
					let button = Button::new(&mut win.add.button_add, Text::new("add vocabulary"));
					if win.add.text_input_question_value != "" && win.add.text_input_answer_value != "" {
						button.on_press(Message::Add(MsgAdd::Add))
					} else {
						button
					}
				})
				.push(Space::new(Length::Fill, Length::Shrink)),
		)
		.push(
			Row::new()
				.push(TextInput::new(
					&mut win.add.text_input_question,
					"question",
					&win.add.text_input_question_value,
					|value| Message::Add(MsgAdd::TextInputQuestion(value)),
				))
				.push(TextInput::new(
					&mut win.add.text_input_answer,
					"answer",
					&win.add.text_input_answer_value,
					|value| Message::Add(MsgAdd::TextInputAnswer(value)),
				)),
		)
		.push(Space::new(Length::Shrink, Length::Units(5)))
		.into()
}
