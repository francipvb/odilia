use std::{collections::HashSet, hash::Hash, rc::Rc};

use crate::odilia_action::OdiliaAction;

/// Odilia object roles.
///
/// These are only for use inside the odilia process.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum OdiliaRole {
	/// Unknown object type.
	Unknown,

	/// A button object.
	///
	/// This describes any kind of buttons, including html elements.
	Button,

	/// Any text box.
	///
	/// This includes single-line, multi-line, read-only and editable text boxes.
	TextBox,

	/// A list of selectable items.
	///
	/// This does not include html lists such numbered or unordered lists.
	ListBox,

	/// Any list item inside a list box.
	///
	/// This does not include html numbered or unordered lists.
	ListItem,

	/// Any tree view.
	TreeView,

	/// An item inside a tree view.
	///
	/// This includes expandable and leaf nodes.
	TreeItem,

	/// Any check box.
	///
	/// This includes html form check boxes.
	CheckBox,

	/// A switch button object.
	///
	/// Not sure if this should be included, but seen in android user interfaces
	/// with it's own role, like in HTML5 aria role enumeration.
	SwitchButton,

	/// A radio button in a radio group.
	///
	/// Often these are grouped without a radio group.
	RadioButton,

	/// Any link object.
	///
	/// This includes html link element.
	Link,

	/// An heading without a level.
	Heading,

	/// Heading of level1.
	Heading1,

	/// Heading of level2.
	Heading2,

	/// Heading of level3.
	Heading3,

	/// Heading of level4.
	Heading4,

	/// Heading of level5.
	Heading5,

	/// Heading of level6.
	Heading6,

	/// Any user interface label.
	///
	/// This includes html label element.
	Label,
}

/// Object states.
///
/// These are used internally only, I.E. they are not intended to be serialized.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum OdiliaState {
	/// Indicates a disabled object.
	Disabled,

	/// Indicates that a text box is multi-linear.
	///
	/// This state should be present only in objects with role [OdiliaRole::TextBox].
	Multiline,

	/// A flag to indicate that an item is read-only.
	///
	/// This state can be included in any editable object whose current state does not allow edition from the user.
	ReadOnly,

	/// A flag to indicate if an object is expandable.
	///
	/// For example, an object with the role set to [OdiliaRole::TreeItem] can be expandable.
	Expandable,

	/// A flag to indicate if an object is expanded.
	///
	/// For example, an expandable object with status of [OdiliaState::Expandable] can have this flag to indicate that the object is expanded.
	Expanded,
	Selected,
	Checked,
	Invalid,
	Required,
	Clickable,
}

pub type OdiliaObjectName = String;
pub type OdiliaObjectDescription = String;

pub type OdiliaObjectId = String;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OdiliaObject {
	role: OdiliaRole,
	states: HashSet<OdiliaState>,
	object_id: OdiliaObjectId,
	name: OdiliaObjectName,
	description: Option<OdiliaObjectDescription>,

	parent: Rc<OdiliaObject>,
	index: u32,
	children: Vec<Rc<OdiliaObject>>,
}

impl OdiliaObject {
	pub fn role(&self) -> &OdiliaRole {
		&self.role
	}

	pub fn states(&self) -> &HashSet<OdiliaState> {
		&self.states
	}

	pub fn object_id(&self) -> &str {
		self.object_id.as_ref()
	}

	pub fn name(&self) -> &str {
		self.name.as_ref()
	}

	pub fn description(&self) -> Option<&String> {
		self.description.as_ref()
	}

	pub fn get_actions(&self) -> &Vec<OdiliaAction<()>> {
		todo!()
	}

	pub async fn execute_action(&self, action_index: u8) -> Result<(), ()> {
		todo!()
	}
}
