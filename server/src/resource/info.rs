use crate::CARGO_PKG_VERSION;
use gotham_restful::{read_all, Resource, Success};
use rusty_vocabulary_models::*;

#[derive(Resource)]
#[resource(info)]
pub struct InfoResource;

#[read_all]
fn info() -> Success<Info> {
	Info {
		version: CARGO_PKG_VERSION.into(),
		about: "rusty-vocabulary".to_owned(),
	}
	.into()
}
