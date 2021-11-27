use gtk::{ButtonExt, Inhibit, OrientableExt, Orientation::Vertical, WidgetExt};
use relm::Widget;
use relm_derive::{widget, Msg};
use std::io;

#[derive(Msg)]
pub enum MsgMainMenu {
	Query,
	Add,
	Explore,
	Quit,
}

#[widget]
impl Widget for WinMainMenu {
	fn model() {}

	view! {
		gtk::Window {
			gtk::Box {
				orientation: Vertical,
				gtk::Button {
					clicked => MsgMainMenu::Query,
					label: "query vocabulary",
				},
				gtk::Button {
					clicked => MsgMainMenu::Add,
					label: "add vocabulary",
				},
				gtk::Button {
					clicked => MsgMainMenu::Explore,
					label: "explore vocabulary",
				},
				gtk::Button {
					clicked => MsgMainMenu::Quit,
					label: "quit",
				},
			},
			delete_event(_, _) => (MsgMainMenu::Quit, Inhibit(false)),
		}
	}

	fn update(&mut self, event: MsgMainMenu) {
		match event {
			MsgMainMenu::Query => eprintln!("query vocabulary is current unimplemented"),
			MsgMainMenu::Add => eprintln!("add vocabulary is current unimplemented"),
			MsgMainMenu::Explore => eprintln!("explore vocabulary is current unimplemented"),
			MsgMainMenu::Quit => gtk::main_quit(),
		}
	}
}

fn main() -> Result<(), io::Error> {
	Win::run(()).unwrap();
	Ok(())
}
