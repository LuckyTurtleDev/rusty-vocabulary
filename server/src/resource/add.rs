use gotham_restful::{create, AuthResult, AuthStatus, Resource};
use rusty_vocabulary_models::{card, Token};

#[derive(Resource)]
#[resource(login)]
pub struct AddResource;

#[create]
fn login(auth: AuthStatus<Token>, card: card::New) -> AuthResult<(), anyhow::Error> {
	warn!("new is still unimplemented");
	let token = auth.ok()?;
	debug!("user {:?} add card: {:?}", token.user_name, card);
	Ok(())
}
