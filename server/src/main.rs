#[macro_use]
extern crate log;

use gotham_restful::{
	create,
	gotham::{self, router::build_simple_router},
	read_all, DrawResources, Resource, Success,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use rusty_vocabulary_models::*;
use simple_logger::SimpleLogger;
use std::time::{SystemTime, UNIX_EPOCH};

const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

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
	debug!("user {:?} has logged in", login.username);
	let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
	eprint!("{:?}", time);
	let claims = Token {
		date: time,
		user_name: login.username.clone(),
		server_version: CARGO_PKG_VERSION.to_owned(),
	};
	let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()));
	match token {
		Err(error) => error!("failed to generate login token for user {:?} : {}", login.username, error),
		Ok(token) => (),
	};
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
