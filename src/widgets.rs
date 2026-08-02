use glam::{Vec2, vec2};

use crate::{Rect, Ui, fnv};

pub struct Button {
	label: String,
	size: Vec2, // NaN = auto, Infinity = max
}

impl Button {
	pub fn new(label: impl Into<String>) -> Self {
		Self { label: label.into(), size: Vec2::NAN }
	}

	/// Take up maximum width
	pub fn wide(mut self) -> Self {
		self.size.x = f32::INFINITY;
		self
	}

	/// Take up maximum height
	pub fn tall(mut self) -> Self {
		self.size.y = f32::INFINITY;
		self
	}

	/// Displays the button and handles input
	pub fn show(&self, ui: &mut Ui) {
		let id = fnv(self.label.as_bytes());
		let width  = if self.size.x < f32::INFINITY {self.size.x} else if self.size.x == f32::INFINITY {ui.rect.width ()} else {50.0};
		let height = if self.size.y < f32::INFINITY {self.size.y} else if self.size.y == f32::INFINITY {ui.rect.height()} else {20.0};
		let rect = Rect::new(ui.cursor, vec2(ui.cursor.x + width, ui.cursor.y + height));
		if ui.horizontal {
			ui.cursor.x += width + 5.0;
		} else {
			ui.cursor.y += height + 5.0;
		}
	}
}