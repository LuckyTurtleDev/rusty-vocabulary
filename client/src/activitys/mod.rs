pub mod add;
pub mod login;
pub mod main_menu;

use add::MsgAdd;
use login::MsgLogin;
use main_menu::MsgMainMenu;

use super::Window;

#[derive(Debug, Clone)]
pub enum Message {
	Login(MsgLogin),
	MainMenu(MsgMainMenu),
	Add(MsgAdd),
}

#[derive(PartialEq)]
pub enum Activity {
	MainMenu,
	Query,
	Add,
	Login,
}
