use crate::{Context, Ui};

pub struct Window {
	title: String,
}

impl Window {
	pub fn new(title: impl Into<String>) -> Self {
		Window { title: title.into() }
	}

	/// Displays the window and handles input
	pub fn show(&self, ctx: &mut Context, add_contents: impl FnOnce(&mut Ui)) {

	}
}