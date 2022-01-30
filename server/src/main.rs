#[macro_use]
extern crate log;

use crate::gotham::pipeline::{new_pipeline, single_pipeline};
use gotham_restful::{
	gotham::{self, router::build_router},
	AuthMiddleware, AuthSource, AuthValidation, DrawResources, StaticAuthHandler,
};
use rusty_vocabulary_models::*;
use simple_logger::SimpleLogger;

mod resource;
use resource::{card::*, info::*, login::*, status::*};

const KEY: &[u8] = b"zlBsA2QXnkmpe0QTh8uCvtAEa4j33YAc";
const EXP: u64 = 63115200; // 2 years

const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
	SimpleLogger::new().env().with_utc_timestamps().init().unwrap();
	let auth: AuthMiddleware<Token, _> = AuthMiddleware::new(
		AuthSource::AuthorizationHeader,
		AuthValidation::default(),
		StaticAuthHandler::from_array(KEY),
	);
	let (chain, pipelines) = single_pipeline(new_pipeline().add(auth).build());
	gotham::start(
		"[::]:8080",
		build_router(chain, pipelines, move |route| {
			route.resource::<InfoResource>("info");
			route.resource::<LoginResource>("login");
			route.resource::<StatusResource>("status");
			route.resource::<CardResource>("card");
		}),
	)
	.unwrap();
}
