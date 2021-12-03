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

enum State {
	MainMenu,
	AddVocabulary,
}

struct Window {
	state: State,
	main_menu: WinMainMenu,
	add_vocabulary: WinAddVocabulary,
}

impl Sandbox for Window {
	type Message = Message;

	fn new() -> Window {
		Window {
			state: State::MainMenu,
			main_menu: WinMainMenu {
				total_vocabulary: 0,
				outstanding_vocabulary: 0,
				subjects: 0,
				button_querry: button::State::new(),
				button_add: button::State::new(),
				button_explore: button::State::new(),
				button_quit: button::State::new(),
			},
			add_vocabulary: WinAddVocabulary {
				button_add: button::State::new(),
				checkbox_both_sides: true,
			},
		}
	}

	fn title(&self) -> String {
		String::from("rusty-vocabulary")
	}

	fn update(&mut self, message: Self::Message) {
		match message {
			Message::MainMenu(msg) => match msg {
				MsgMainMenu::Quit => exit(0),
				MsgMainMenu::Add => self.state = State::AddVocabulary,
			},
			Message::AddVocabulary(msg) => match msg {
				MsgAddVocabulary::CheckboxBothSidesToogle => {
					self.add_vocabulary.checkbox_both_sides = !self.add_vocabulary.checkbox_both_sides
				},
			},
		};
	}

	fn view(&mut self) -> Element<Self::Message> {
		match self.state {
			State::MainMenu => Row::new()
				.push(Space::new(Length::Fill, Length::Shrink))
				.push(
					Column::new()
						.push(Space::new(Length::Shrink, Length::Fill))
						.push(Text::new(format!("total vocabulary: {}", self.main_menu.total_vocabulary))) //todo: use //align_items(Alignment::Fill)
						.push(Text::new(format!("outstanding vocabulary: {}", self.main_menu.outstanding_vocabulary)))
						.push(Text::new(format!("subjects/languages: {}", self.main_menu.subjects)))
						.push(Space::new(Length::Shrink, Length::Fill)),
				)
				.push(Space::new(iced::Length::Units(20), Length::Shrink))
				.push(
					Column::new()
						.push(Space::new(Length::Shrink, Length::Fill))
						.push(Button::new(&mut self.main_menu.button_querry, Text::new("querry vocabulary")))
						.push(
							Button::new(&mut self.main_menu.button_add, Text::new("add vocabulary"))
								.on_press(Message::MainMenu(MsgMainMenu::Add)),
						)
						.push(Button::new(
							&mut self.main_menu.button_explore,
							Text::new("explore vocabulary"),
						))
						.push(
							Button::new(&mut self.main_menu.button_quit, Text::new("quit"))
								.on_press(Message::MainMenu(MsgMainMenu::Quit)),
						)
						.push(Space::new(Length::Shrink, Length::Fill)),
				)
				.push(Space::new(Length::Fill, Length::Shrink))
				.into(),
			State::AddVocabulary => Column::new()
				.push(
					Row::new()
						.push(Text::new("subject/language"))
						.push(Checkbox::new(self.add_vocabulary.checkbox_both_sides, "bot sides", |_| {
							Message::AddVocabulary(MsgAddVocabulary::CheckboxBothSidesToogle)
						}))
						.push(Button::new(&mut self.add_vocabulary.button_add, Text::new("add vocabulary"))),
				)
				.into(),
		}
	}
}

fn main() -> iced::Result {
	Window::run(Settings::default())
}
