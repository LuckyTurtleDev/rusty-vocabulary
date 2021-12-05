use iced::{
	button, text_input, widget::Space, Align, Button, Checkbox, Column, Element, Length, Row, Sandbox, Settings, Text,
	TextInput,
};
use std::process::exit;

#[derive(Debug, Clone, Copy)]
pub enum MsgMainMenu {
	Add,
	Quit,
}

#[derive(Debug, Clone)]
pub enum MsgAddVocabulary {
	Back,
	TextInputTags(String),
	CheckboxBothSidesToogle,
}

#[derive(Debug, Clone)]
pub enum Message {
	MainMenu(MsgMainMenu),
	AddVocabulary(MsgAddVocabulary),
}

struct WinAddVocabulary {
	button_back: button::State,
	text_input_tags: text_input::State,
	text_input_tags_value: String,
	checkbox_both_sides: bool,
	button_add: button::State,
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

enum Activity {
	MainMenu,
	AddVocabulary,
}

struct Window {
	activity: Activity,
	main_menu: WinMainMenu,
	add_vocabulary: WinAddVocabulary,
}

impl Sandbox for Window {
	type Message = Message;

	fn new() -> Window {
		Window {
			activity: Activity::MainMenu,
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
				button_back: button::State::new(),
				text_input_tags: text_input::State::new(),
				text_input_tags_value: String::new(),
				checkbox_both_sides: true,
				button_add: button::State::new(),
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
				MsgMainMenu::Add => self.activity = Activity::AddVocabulary,
			},
			Message::AddVocabulary(msg) => match msg {
				MsgAddVocabulary::Back => self.activity = Activity::MainMenu,
				MsgAddVocabulary::TextInputTags(value) => self.add_vocabulary.text_input_tags_value = value,
				MsgAddVocabulary::CheckboxBothSidesToogle => {
					self.add_vocabulary.checkbox_both_sides = !self.add_vocabulary.checkbox_both_sides
				},
			},
		};
	}

	fn view(&mut self) -> Element<Self::Message> {
		match self.activity {
			Activity::MainMenu => Row::new()
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
						.push(Button::new(&mut self.add_vocabulary.button_add, Text::new("add vocabulary")))
						.push(Space::new(Length::Fill, Length::Shrink)),
				)
				.push(Space::new(Length::Shrink, Length::Units(5)))
				.into(),
		}
	}
}

fn main() -> iced::Result {
	Window::run(Settings::default())
}
