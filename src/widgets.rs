use glam::{Vec2, vec2};

use crate::{Rect, Ui, Response, fnv};

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

	/// Set size
	pub fn size(mut self, size: Vec2) -> Self {
		self.size = size;
		self
	}

	/// Displays the button and handles input
	pub fn show(&self, ui: &mut Ui) -> Response {
		let id = fnv(self.label.as_bytes());
		let size = vec2(
			if self.size.x < f32::INFINITY {self.size.x} else if self.size.x == f32::INFINITY {ui.rect.width ()} else {50.0},
			if self.size.y < f32::INFINITY {self.size.y} else if self.size.y == f32::INFINITY {ui.rect.height()} else {20.0}
		);
		let (rect, response) = ui.add_rect(size);
		ui.ctx.draw_list.add_rect(rect, ui.ctx.colors.button);
		response
	}
}