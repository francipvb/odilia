use std::sync::Arc;
use crate::{
	state::ScreenReaderState,
	traits::{IntoOdiliaCommands, IntoStateView, Command, StateView, MutableStateView, IntoMutableStateView},
};
use async_trait::async_trait;
use atspi_common::events::RemoveAccessibleEvent;
use atspi_common::State;
use odilia_common::events::{ScreenReaderEvent};
use odilia_common::{
	cache::ExternalCacheItem,
	errors::{OdiliaError, CacheError},
	commands::{OdiliaCommand, RemoveItemCommand},
};
use odilia_cache::{CacheRef, CacheValue, CacheItem, Cache};

#[async_trait]
impl IntoOdiliaCommands for RemoveAccessibleEvent {
	async fn commands(&self, _: &<Self as StateView>::View) -> Result<Vec<OdiliaCommand>, OdiliaError> {
		Ok(vec![
			RemoveItemCommand {
				item: self.item.clone().into()
			}.into()
		].into())
	}
}

impl MutableStateView for RemoveItemCommand {
	type View = Arc<Cache>;
}

#[async_trait]
impl IntoMutableStateView for RemoveItemCommand {
	async fn create_view(&self, state: &ScreenReaderState) -> Result<<Self as MutableStateView>::View, OdiliaError> {
		Ok(Arc::clone(&state.cache))
	}
}

#[async_trait]
impl Command for RemoveItemCommand {
	async fn execute(&self, cache: <Self as MutableStateView>::View) -> Result<(), OdiliaError> {
		let _ = cache.remove(&self.item).await;
		Ok(())
	}
}
