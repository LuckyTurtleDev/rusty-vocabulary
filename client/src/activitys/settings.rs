use super::{Activity, Message};
use crate::{config::ThemeTitel, save_config};
use iced::{button, pick_list, widget::Space, Align, Button, Column, Container, Element, Length, PickList, Text};

#[derive(Debug, Clone, PartialEq)]
pub enum MsgSettings {
	ThemeSelected(ThemeTitel),
	Back,
}

#[derive(Default)]
pub struct WinSettings {
	pub pick_list_theme: pick_list::State<ThemeTitel>,
	pub pick_list_theme_value: ThemeTitel,
	pub button_back: button::State,
}

pub fn new() -> WinSettings {
	WinSettings::default()
}

pub fn update(win: &mut super::Window, message: MsgSettings) {
	match message {
		MsgSettings::ThemeSelected(theme_title) => {
			win.settings.pick_list_theme_value = theme_title;
			match theme_title {
				ThemeTitel::Light => win.theme = typetest_themes::Theme::DefaultLight.into(),
				ThemeTitel::Dark => win.theme = typetest_themes::Theme::DefaultDark.into(),
			}
		},
		MsgSettings::Back => win.activity = Activity::MainMenu,
	}
	if message != MsgSettings::Back {
		save_config(&win.config);
	}
}

pub fn view(win: &mut super::Window) -> Element<super::Message> {
	let content = Column::new()
		.align_items(Align::Center)
		.push(Text::new("Settings").size(40))
		.align_items(Align::Start)
		.push(Space::with_height(Length::Units(40)))
		.push(Text::new("Theme:"))
		.align_items(Align::End)
		.push(
			PickList::new(
				&mut win.settings.pick_list_theme,
				vec![ThemeTitel::Light, ThemeTitel::Dark],
				Some(win.settings.pick_list_theme_value),
				|value| Message::Settings(MsgSettings::ThemeSelected(value)),
			)
			.style(&win.theme),
		)
		.push(Space::with_height(Length::Units(40)))
		.align_items(Align::Center)
		.push(
			Button::new(&mut win.settings.button_back, Text::new("back"))
				.on_press(Message::Settings(MsgSettings::Back))
				.style(&win.theme),
		);

	Container::new(content)
		.width(Length::Fill)
		.height(Length::Fill)
		.center_x()
		.center_y()
		.style(&win.theme)
		.into()
}
