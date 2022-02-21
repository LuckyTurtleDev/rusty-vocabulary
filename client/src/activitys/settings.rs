use iced::{pick_list, Column, Container, Element, Length, PickList, Text};
use strum_macros::Display;

#[derive(Debug, Clone)]
pub enum MsgSettings {
	ThemeSelected(ThemeTitel),
}

#[derive(Debug, Clone, Copy, Display, PartialEq, Eq)]
#[strum(serialize_all = "lowercase")]
pub enum ThemeTitel {
	Light,
	Dark,
}

impl Default for ThemeTitel {
	fn default() -> ThemeTitel {
		ThemeTitel::Dark
	}
}

#[derive(Default)]
pub struct WinSettings {
	pub pick_list_theme: pick_list::State<ThemeTitel>,
	pub pick_list_theme_value: ThemeTitel,
}

pub fn new() -> WinSettings {
	WinSettings::default()
}

pub fn update(win: &mut super::Window, message: MsgSettings) {}

pub fn view(win: &mut super::Window) -> Element<super::Message> {
	let content = Column::new().push(Text::new("Theme:")).push(PickList::new(
		&mut win.settings.pick_list_theme,
		vec![ThemeTitel::Light, ThemeTitel::Dark],
		Some(win.settings.pick_list_theme_value),
		MsgSettings::ThemeSelected,
	));

	Container::new(content)
		.width(Length::Fill)
		.height(Length::Fill)
		.center_x()
		.center_y()
		.style(&win.theme)
		.into() //https://github.com/iced-rs/iced/blob/06517aa7e83b92f22795046bfd1f78402285d62f/examples/pick_list/src/main.rs#L42
}
