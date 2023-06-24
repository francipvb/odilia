//! State management for Odilia.
//! This has a bunch of smaller structures for handling the minimum state necessary to produce a command.
//! Please see the information on the Odilia architecture in the `README.md`.

use crate::cache::{CacheItem, CacheKey};
use crate::traits::StateView;
use atspi_common::events::{
	object::{
		TextCaretMovedEvent,
		TextChangedEvent,
		StateChangedEvent,
	},
};

use serde::{Serialize, Deserialize};

macro_rules! impl_state_view {
	($type:ty, $state_view:ty) => {
		impl StateView for $type {
			type View = $state_view;
		}
	}
}

/// View for a caret position change event.
#[derive(Serialize, Deserialize, Clone)]
pub struct CaretPositionView {
	/// The previous position of the curosr.
	previous_position: i32,
	/// The previously focused item.
	previous_focus: CacheKey,
}
impl_state_view!(TextCaretMovedEvent, CaretPositionView);

impl_state_view!(TextChangedEvent, CacheItem);
impl_state_view!(StateChangedEvent, CacheItem);
