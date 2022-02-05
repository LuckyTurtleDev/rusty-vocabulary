use gotham_restful::{read_all, AuthResult, AuthStatus, Resource};
use rusty_vocabulary_models::{card, Token};

#[derive(Resource)]
#[resource(card)]
pub struct CardResource;

#[read_all]
fn card(auth: AuthStatus<Token>) -> AuthResult<Vec<card::Small>, anyhow::Error> {
	warn!("card is still unimplemented");
	let token = auth.ok()?;
	let card = card::Small {
		id: 42,
		content: card::Content {
			question: "This is a question?".into(),
			answer: "and this is the answer".into(),
		},
	};
	Ok(vec![card])
}
