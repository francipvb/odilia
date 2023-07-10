use std::{
	collections::{btree_set::Iter, HashSet},
	rc::Rc,
};

use crate::odilia_object::{OdiliaObject, OdiliaState};

#[derive(Clone)]
pub struct OdiliaEvent {
	event_type: OdiliaEventType,
	object: Rc<OdiliaObject>,
}

impl OdiliaEvent {
	pub fn new(event_type: OdiliaEventType, object: &OdiliaObject) -> Self {
		let saved_object: OdiliaObject = object.clone();
		Self { event_type, object: Rc::new(saved_object) }
	}

	pub fn event_type(&self) -> &OdiliaEventType {
		&self.event_type
	}

	pub fn object(&self) -> &OdiliaObject {
		self.object.as_ref()
	}
}
#[derive(Clone, PartialEq, Eq)]
pub enum OdiliaEventType {
	StateChanged(StateChangedData),
	CaretChanged(CaretChangedData),
	FocusChanged(FocusChangedData),
}

#[derive(Clone, PartialEq, Eq)]
pub struct StateChangedData {
	previous_states: HashSet<OdiliaState>,
	new_states: HashSet<OdiliaState>,
}

impl StateChangedData {
	pub fn new(previous_states: Iter<OdiliaState>, new_states: Iter<OdiliaState>) -> Self {
		Self {
			previous_states: previous_states
				.map(|s| s.clone())
				.collect::<HashSet<OdiliaState>>(),
			new_states: new_states.map(|s| s.clone()).collect(),
		}
	}

	pub fn previous_state(&self) -> &HashSet<OdiliaState> {
		&self.previous_states
	}

	pub fn new_states(&self) -> &HashSet<OdiliaState> {
		&self.new_states
	}
}

#[derive(Clone, PartialEq, Eq)]
pub struct FocusChangedData {
	previous_object: Rc<OdiliaObject>,
}

impl FocusChangedData {
	pub fn new(previous_object: &OdiliaObject) -> Self {
		Self { previous_object: Rc::new(previous_object.clone()) }
	}

	pub fn previous_object(&self) -> &OdiliaObject {
		self.previous_object.as_ref()
	}
}

pub type CaretPosition = u32;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct CaretChangedData {
	previous_position: CaretPosition,
	current_position: CaretPosition,
}

impl CaretChangedData {
	pub fn new(previous_position: CaretPosition, current_position: CaretPosition) -> Self {
		Self { previous_position, current_position }
	}

	pub fn previous_position(&self) -> u32 {
		self.previous_position
	}

	pub fn current_position(&self) -> u32 {
		self.current_position
	}
}

pub struct TextSelection(CaretPosition, CaretPosition);

impl TextSelection {
	pub fn new(start: CaretPosition, end: CaretPosition) -> Self {
		Self(start, end)
	}

	pub fn start(&self) -> CaretPosition {
		self.0
	}
	pub fn end(&self) -> CaretPosition {
		self.1
	}
}

pub struct TextSelectionData {
	previous: TextSelection,
	current: TextSelection,
}

impl TextSelectionData {
	pub fn current(&self) -> &TextSelection {
		&self.current
	}

	pub fn previous(&self) -> &TextSelection {
		&self.previous
	}
}
