use super::CARGO_PKG_NAME;
use native_dialog::{MessageDialog, MessageType};
use std::process::exit;

pub trait UnwrapGui<T> {
	fn expect_gui(self, msg: &str) -> T;
}

impl<T> UnwrapGui<T> for Option<T> {
	#[inline]
	#[track_caller]
	fn expect_gui(self, msg: &str) -> T {
		match self {
			Some(value) => value,
			None => gui_panic(msg),
		}
	}
}

#[track_caller]
pub fn gui_exit_with_error(msg: &str, exit_code: i32) {
	let msg = &format!("Error: {}", msg);
	let res = MessageDialog::new()
		.set_type(MessageType::Error)
		.set_title("Error")
		.set_text(msg)
		.show_alert();
	if res.is_err() {
		eprintln!("Error showing error popup: {}", res.unwrap_err())
	}
	eprintln!("{}", msg);
	exit(exit_code);
}

#[track_caller]
pub fn gui_panic(msg: &str) -> ! {
	let msg = &format!("{} panicked: {}", CARGO_PKG_NAME, msg);
	let res = MessageDialog::new()
		.set_type(MessageType::Error)
		.set_title("panicked")
		.set_text(msg)
		.show_alert();
	if res.is_err() {
		eprintln!("Error showing error popup: {}", res.unwrap_err())
	}
	panic!("{}", msg);
}
