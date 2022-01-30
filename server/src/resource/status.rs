use gotham_restful::{read_all, AuthResult, AuthStatus, Resource};
use rusty_vocabulary_models::*;

#[derive(Resource)]
#[resource(status)]
pub struct StatusResource;

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
