#[macro_use]
extern crate log;

use gotham_restful::{
	create,
	gotham::{self, prelude::*, router::build_simple_router},
	read_all, DrawResources, Resource, Success,
};
use rusty_vocabulary_models::*;
use simple_logger::SimpleLogger;

fn main() {
	SimpleLogger::new().env().with_utc_timestamps().init().unwrap();
	gotham::start(
		"[::]:8080",
		build_simple_router(move |route| {
			route.resource::<InfoResource>("info");
			route.resource::<AccountResource>("login");
		}),
	)
	.unwrap();
}

#[derive(Resource)]
#[resource(login)]
struct AccountResource;

#[create]
fn login(login: Login) {
	info!("user {:?} has logged in with password {:?}", login.username, login.password);
}

#[derive(Resource)]
#[resource(info)]
struct InfoResource;

#[read_all]
fn info() -> Success<Info> {
	Info {
		version: env!("CARGO_PKG_VERSION").to_owned(),
		about: "rusty-vocabulary".to_owned(),
	}
	.into()
}
