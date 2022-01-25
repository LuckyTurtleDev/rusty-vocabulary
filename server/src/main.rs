#[macro_use]
extern crate log;

use crate::gotham::pipeline::{new_pipeline, single_pipeline};
use gotham_restful::{
	create,
	gotham::{self, router::build_router},
	read_all, AuthError, AuthMiddleware, AuthResult, AuthSource, AuthStatus, AuthValidation, DrawResources, Resource,
	ResourceError, StaticAuthHandler, Success,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use rusty_vocabulary_models::*;
use simple_logger::SimpleLogger;
use std::time::{SystemTime, UNIX_EPOCH};

const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
	SimpleLogger::new().env().with_utc_timestamps().init().unwrap();
	let auth: AuthMiddleware<Token, _> = AuthMiddleware::new(
		AuthSource::AuthorizationHeader,
		AuthValidation::default(),
		StaticAuthHandler::from_array(b"zlBsA2QXnkmpe0QTh8uCvtAEa4j33YAc"),
	);
	let (chain, pipelines) = single_pipeline(new_pipeline().add(auth).build());
	gotham::start(
		"[::]:8080",
		build_router(chain, pipelines, move |route| {
			route.resource::<InfoResource>("info");
			route.resource::<LoginResource>("login");
			route.resource::<StatusResource>("status");
		}),
	)
	.unwrap();
}

#[derive(Resource)]
#[resource(login)]
struct LoginResource;

#[derive(Debug, ResourceError)]
pub enum LoginError {
	#[status(INTERNAL_SERVER_ERROR)]
	#[display("Internal Server Error")]
	InternalServerError,
}

#[create]
fn login(login: Login) -> Result<String, LoginError> {
	debug!("user {:?} has logged in", login.username);
	warn!("login is still unimplemented");
	let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
	let claims = Token {
		iat: time,
		user_name: login.username.clone(),
		server_version: CARGO_PKG_VERSION.to_owned(),
	};
	let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()));
	match token {
		Err(error) => {
			error!("failed to generate login token for user {:?} : {}", login.username, error);
			Err(LoginError::InternalServerError)
		},
		Ok(token) => Ok(token),
	}
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

#[derive(Resource)]
#[resource(status)]
struct StatusResource;

#[read_all]
fn status(auth: AuthStatus<Token>) -> AuthResult<Status, anyhow::Error> {
	warn!("status is still unimplemented");
	let token = auth.ok()?;
	let status = Status {
		vocabulary: 42,
		outstanding_vocabulary: 42,
		subjects: 42,
		outstanding_subjects: 42,
	};
	Ok(status)
}
