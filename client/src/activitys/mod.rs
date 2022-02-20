pub mod add;
pub mod login;
pub mod main_menu;
pub mod settings;

use add::MsgAdd;
use login::MsgLogin;
use main_menu::MsgMainMenu;
use settings::MsgSettings;

use super::Window;

#[derive(Debug, Clone)]
pub enum Message {
	Login(MsgLogin),
	MainMenu(MsgMainMenu),
	Add(MsgAdd),
	Settings(MsgSettings),
}

#[derive(PartialEq)]
pub enum Activity {
	MainMenu,
	Query,
	Add,
	Settings,
	Login,
}
