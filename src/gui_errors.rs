use super::CARGO_PKG_NAME;
use native_dialog::{MessageDialog, MessageType};
use std::{fmt::Debug, process::exit};

pub trait UnwrapGui<T> {
	fn expect_gui(self, msg: &str) -> T;
	fn expect_gui_exit(self, msg: &str, error_code: i32) -> T;
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

	#[inline]
	#[track_caller]
	fn expect_gui_exit(self, msg: &str, error_code: i32) -> T {
		match self {
			Some(value) => value,
			None => gui_exit_with_error(msg, error_code),
		}
	}
}

impl<T, E: Debug> UnwrapGui<T> for Result<T, E> {
	#[inline]
	#[track_caller]
	fn expect_gui(self, msg: &str) -> T {
		match self {
			Ok(value) => value,
			Err(error) => gui_panic(&format!("{}:\n {:?}", msg, error)),
		}
	}

	#[inline]
	#[track_caller]
	fn expect_gui_exit(self, msg: &str, error_code: i32) -> T {
		match self {
			Ok(value) => value,
			Err(error) => gui_exit_with_error(&format!("{}:\n {:?}", msg, error), error_code),
		}
	}
}

#[track_caller]
pub fn gui_exit_with_error(msg: &str, exit_code: i32) -> ! {
	let res = MessageDialog::new()
		.set_type(MessageType::Error)
		.set_title("Error")
		.set_text(msg)
		.show_alert();
	if res.is_err() {
		eprintln!("Error showing error popup: {}", res.unwrap_err())
	}
	eprintln!("Error: {msg}");
	exit(exit_code);
}

#[track_caller]
pub fn gui_panic(msg: &str) -> ! {
	let res = MessageDialog::new()
		.set_type(MessageType::Error)
		.set_title("panicked")
		.set_text(&format!("{CARGO_PKG_NAME} panicked:\n {msg}"))
		.show_alert();
	if res.is_err() {
		eprintln!("Error showing error popup: {}", res.unwrap_err());
	}
	panic!("{msg}");
}

#[track_caller]
pub fn unimplemented() -> ! {
	let res = MessageDialog::new()
		.set_type(MessageType::Error)
		.set_title("unimplemented")
		.set_text("this code/feature is unimplemented")
		.show_alert();
	if res.is_err() {
		eprintln!("Error showing error popup: {}", res.unwrap_err());
	}
	unimplemented!();
}
