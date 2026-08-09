use std::ops::Deref;

use glam::Vec2;

use crate::{Context, Rect, Response, widgets::Button};

pub struct Ui<'a> {
	pub ctx: &'a mut Context,
	pub rect: Rect,
	pub cursor: Vec2,
	pub horizontal: bool,
}

impl Ui<'_> {
	/// Draws some text
	pub fn label(&mut self, text: impl Into<String>) {
		
	}
	/// Shortcut for Button::new(label).show(ui).clicked
	pub fn button(&mut self, label: impl Into<String>) -> bool {
		Button::new(label).show(self).clicked
	}

	/// Creates a rectangle at the cursor with a certain size and returns the rectangle and some input data
	pub fn add_rect(&mut self, size: Vec2) -> (Rect, Response) {
		let rect = Rect::sized(self.cursor, size);
		if self.horizontal {
			self.cursor.x += size.x + 5.0;
		} else {
			self.cursor.y += size.y + 5.0;
		}
		(rect, Response { clicked: false, holding: false, hovered: false })
	}
}