use iced::{button, widget::Space, Button, Checkbox, Column, Element, Length, Row, Sandbox, Settings, Text};
use std::process::exit;

#[derive(Debug, Clone, Copy)]
pub enum MsgMainMenu {
	Add,
	Quit,
}

#[derive(Debug, Clone, Copy)]
pub enum MsgAddVocabulary {
	CheckboxBothSidesToogle,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
	MainMenu(MsgMainMenu),
	AddVocabulary(MsgAddVocabulary),
}

struct WinAddVocabulary {
	button_add: button::State,
	checkbox_both_sides: bool,
}

struct WinMainMenu {
	total_vocabulary: u32,
	outstanding_vocabulary: u32,
	subjects: u16,
	button_querry: button::State,
	button_add: button::State,
	button_explore: button::State,
	button_quit: button::State,
}

enum Window {
	MainMenu(WinMainMenu),
	AddVocabulary(WinAddVocabulary),
}

impl Sandbox for Window {
	type Message = Message;

	fn new() -> Window {
		Window::MainMenu(WinMainMenu {
			total_vocabulary: 0,
			outstanding_vocabulary: 0,
			subjects: 0,
			button_querry: button::State::new(),
			button_add: button::State::new(),
			button_explore: button::State::new(),
			button_quit: button::State::new(),
		})
	}

	fn title(&self) -> String {
		String::from("rusty-vocabulary")
	}

	fn update(&mut self, message: Self::Message) {
		match message {
			Message::MainMenu(msg) => match msg {
				MsgMainMenu::Quit => exit(0),
				MsgMainMenu::Add => {
					*self = Window::AddVocabulary(WinAddVocabulary {
						button_add: button::State::new(),
						checkbox_both_sides: true,
					})
				},
			},
			Message::AddVocabulary(msg) => match msg {
				MsgAddVocabulary::CheckboxBothSidesToogle => match self {
					Window::AddVocabulary(state) => state.checkbox_both_sides = !state.checkbox_both_sides,
					_ => panic!("Application is at the wrong state for this event"),
				},
			},
		};
	}

	fn view(&mut self) -> Element<Self::Message> {
		match self {
			Window::MainMenu(state) => Row::new()
				.push(Space::new(Length::Fill, Length::Shrink))
				.push(
					Column::new()
						.push(Space::new(Length::Shrink, Length::Fill))
						.push(Text::new(format!("total vocabulary: {}", state.total_vocabulary))) //todo: use //align_items(Alignment::Fill)
						.push(Text::new(format!("outstanding vocabulary: {}", state.outstanding_vocabulary)))
						.push(Text::new(format!("subjects/languages: {}", state.subjects)))
						.push(Space::new(Length::Shrink, Length::Fill)),
				)
				.push(Space::new(iced::Length::Units(20), Length::Shrink))
				.push(
					Column::new()
						.push(Space::new(Length::Shrink, Length::Fill))
						.push(Button::new(&mut state.button_querry, Text::new("querry vocabulary")))
						.push(
							Button::new(&mut state.button_add, Text::new("add vocabulary"))
								.on_press(Message::MainMenu(MsgMainMenu::Add)),
						)
						.push(Button::new(&mut state.button_explore, Text::new("explore vocabulary")))
						.push(
							Button::new(&mut state.button_quit, Text::new("quit"))
								.on_press(Message::MainMenu(MsgMainMenu::Quit)),
						)
						.push(Space::new(Length::Shrink, Length::Fill)),
				)
				.push(Space::new(Length::Fill, Length::Shrink))
				.into(),
			Window::AddVocabulary(state) => Column::new()
				.push(
					Row::new()
						.push(Text::new("subject/language"))
						.push(Checkbox::new(state.checkbox_both_sides, "bot sides", |_| {
							Message::AddVocabulary(MsgAddVocabulary::CheckboxBothSidesToogle)
						}))
						.push(Button::new(&mut state.button_add, Text::new("add vocabulary"))),
				)
				.into(),
		}
	}
}

fn main() -> iced::Result {
	Window::run(Settings::default())
}
