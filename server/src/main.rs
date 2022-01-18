use gotham_restful::gotham::{self, prelude::*, router::build_simple_router};
use gotham_restful::{read_all, DrawResources, Resource, Success};
use log::warn;
use serde::Serialize;
use simple_logger::SimpleLogger;

fn main() {
	SimpleLogger::new().env().with_utc_timestamps().init().unwrap();
	warn!("This is an example message.");
	gotham::start(
		"[::]:8080",
		build_simple_router(move |route| {
			route.resource::<InfoResource>("info");
		}),
	)
	.unwrap();
}

#[derive(Resource)]
#[resource(info)]
struct InfoResource;

#[derive(Serialize)]
struct Info {
	version: String,
	about: String,
}

#[read_all]
fn info() -> Success<Info> {
	Info {
		version: env!("CARGO_PKG_VERSION").to_owned(),
		about: "rusty-vocabulary".to_owned(),
	}
	.into()
}
