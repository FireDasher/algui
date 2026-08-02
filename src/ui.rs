use std::ops::Deref;

use glam::Vec2;

use crate::{Context, Rect};

pub struct Ui {
	context: &'static mut Context,
	pub rect: Rect,
	pub cursor: Vec2,
	pub horizontal: bool,
}

// Allow using the ui as a context
impl Deref for Ui {
	type Target = Context;
	fn deref(&self) -> &Self::Target {
    	self.context
	}
}