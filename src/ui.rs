use glam::{Vec2, vec2};

use crate::{Context, Rect, Response, id::id_from_str};

pub struct Ui<'a> {
	pub ctx: &'a mut Context,
	pub rect: Rect,
	pub cursor: Vec2,
	pub horizontal: bool,
}

impl Ui<'_> {
	/// Draws some text
	pub fn label(&mut self, text: &str) {
		let size = vec2(text.len() as f32 * 8.0, 14.0);
		let rect = Rect::sized(self.cursor, size);
		self.ctx.draw_list.add_rect(rect, self.ctx.colors.text);
		if self.horizontal {
			self.cursor.x += size.x + 5.0;
		} else {
			self.cursor.y += size.y + 5.0;
		}
	}

	/// Shows a button and checks for input
	pub fn button(&mut self, label: &str) -> Response {
		// let id = id_from_str(label);
		let size = vec2(50.0, 20.0);
		let (rect, response) = self.add_rect(size);
		self.ctx.draw_list.add_rect(rect, self.ctx.colors.button);
		response
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