pub type ActionId = String;
pub type ActionHelp = String;

#[async_trait::async_trait]
pub trait ActionHandler {
	type Error;

	async fn handle(&self) -> Result<(), Self::Error>;
}

pub struct OdiliaAction<T>
where
	T: ActionHandler,
{
	action_id: ActionId,
	action_help: ActionHelp,
	handler: &ActionHandler,
}
