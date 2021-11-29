use iced::{button, widget::Space, Button, Column, Element, Length, Row, Sandbox, Settings, Text};
use std::process::exit;

#[derive(Debug, Clone, Copy)]
pub enum MsgMainMenu {
	Add,
	Quit,
	Nothing,
}

enum AktiveMenu {
	MainMenu,
	Add,
}

struct AddWindow {
	button_add: button::State,
}

struct MainMenu {
	aktive_menu: AktiveMenu,
	total_vocabulary: u32,
	outstanding_vocabulary: u32,
	subjects: u16,
	button_querry: button::State,
	button_add: button::State,
	button_explore: button::State,
	button_quit: button::State,
	add: AddWindow,
}

impl Sandbox for MainMenu {
	type Message = MsgMainMenu;

	fn new() -> MainMenu {
		MainMenu {
			aktive_menu: AktiveMenu::MainMenu,
			total_vocabulary: 0,
			outstanding_vocabulary: 0,
			subjects: 0,
			button_querry: button::State::new(),
			button_add: button::State::new(),
			button_explore: button::State::new(),
			button_quit: button::State::new(),
			add: AddWindow {
				button_add: button::State::new(),
			},
		}
	}

	fn title(&self) -> String {
		String::from("rusty-vocabulary")
	}

	fn update(&mut self, message: Self::Message) {
		match message {
			MsgMainMenu::Quit => exit(0),
			MsgMainMenu::Add => self.aktive_menu = AktiveMenu::Add,
			MsgMainMenu::Nothing => (),
		};
	}

	fn view(&mut self) -> Element<Self::Message> {
		match self.aktive_menu {
			AktiveMenu::MainMenu => Row::new()
				.push(Space::new(Length::Fill, Length::Shrink))
				.push(
					Column::new()
						.push(Space::new(Length::Shrink, Length::Fill))
						.push(Text::new(format!("total vocabulary: {}", self.total_vocabulary)))
						.push(Text::new(format!("outstanding vocabulary: {}", self.outstanding_vocabulary)))
						.push(Text::new(format!("subjects/languages: {}", self.subjects)))
						.push(Space::new(Length::Shrink, Length::Fill)),
				)
				.push(Space::new(iced::Length::Units(20), Length::Shrink))
				.push(
					Column::new()
						.push(Space::new(Length::Shrink, Length::Fill))
						.push(Button::new(&mut self.button_querry, Text::new("querry vocabulary")))
						.push(Button::new(&mut self.button_add, Text::new("add vocabulary")).on_press(MsgMainMenu::Add))
						.push(Button::new(&mut self.button_explore, Text::new("explore vocabulary")))
						.push(Button::new(&mut self.button_quit, Text::new("quit")).on_press(MsgMainMenu::Quit))
						.push(Space::new(Length::Shrink, Length::Fill)),
				)
				.push(Space::new(Length::Fill, Length::Shrink))
				.into(),
			AktiveMenu::Add => Column::new()
				.push(
					Row::new()
						.push(Text::new("subject/language"))
						.push(Text::new("both side"))
						.push(Button::new(&mut self.add.button_add, Text::new("add vocabulary"))),
				)
				.into(),
		}
	}
}

fn main() -> iced::Result {
	MainMenu::run(Settings::default())
}
