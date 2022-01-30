use crate::{EXP, KEY};
use gotham_restful::{create, Raw, Resource, ResourceError};
use jsonwebtoken::{encode, EncodingKey, Header};
use rusty_vocabulary_models::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Resource)]
#[resource(login)]
pub struct LoginResource;

#[derive(Debug, ResourceError)]
pub enum LoginError {
	#[status(INTERNAL_SERVER_ERROR)]
	#[display("Internal Server Error")]
	InternalServerError,
}

#[create]
fn login(login: Login) -> Result<Raw<String>, LoginError> {
	debug!("user {:?} has logged in", login.username);
	warn!("login is still unimplemented");
	let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
	let claims = Token {
		iat: time,
		exp: time + EXP,
		user_name: login.username.clone(),
	};
	let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(KEY));
	match token {
		Err(error) => {
			error!("failed to generate login token for user {:?} : {}", login.username, error);
			Err(LoginError::InternalServerError)
		},
		Ok(token) => Ok(Raw {
			raw: token,
			mime: mime::TEXT_PLAIN,
		}),
	}
}
