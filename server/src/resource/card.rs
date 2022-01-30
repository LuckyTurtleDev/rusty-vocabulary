use gotham_restful::{read_all, AuthResult, AuthStatus, Resource};
use rusty_vocabulary_models::*;

#[derive(Resource)]
#[resource(card)]
pub struct CardResource;

#[read_all]
fn card(auth: AuthStatus<Token>) -> AuthResult<Vec<CardPartial>, anyhow::Error> {
	warn!("card is still unimplemented");
	let token = auth.ok()?;
	let card = CardPartial {
		question: "This is a question?".into(),
		answer: "and this is the answer".into(),
		id: 42,
	};
	Ok(vec![card])
}
