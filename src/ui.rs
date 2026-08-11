use glam::{Vec2, vec2};

use crate::{Context, Rect, Response, id::id_from_str};

pub struct Ui<'a> {
	pub ctx: &'a mut Context,
	pub rect: Rect,
	pub cursor: Vec2,
	pub horizontal_cursor: Vec2,
}

impl<'a> Ui<'a> {
	/// Creates a new Ui inside a rectangle
	pub fn new(ctx: &'a mut Context, rect: Rect) -> Self {
		Self {ctx, rect, cursor: rect.min, horizontal_cursor: rect.min}
	}

	/// Moves the cursor by a certain amount
	pub fn move_cursor(&mut self, amount: Vec2) {
		self.horizontal_cursor = self.cursor;
		self.horizontal_cursor.x += amount.x + 5.0;
		self.cursor.x = self.rect.min.x;
		self.cursor.y += amount.y + 5.0;
	}

	/// Draw the next element on the same line
	pub fn same_line(&mut self) {
		self.cursor = self.horizontal_cursor;
	}

	/// Draws some text
	pub fn label(&mut self, text: &str) {
		let size = vec2(text.len() as f32 * 8.0, 14.0);
		let rect = Rect::sized(self.cursor, size);
		self.ctx.draw_list.add_rect(rect, self.ctx.colors.text);
		self.move_cursor(size);
	}

	/// Shows a button and checks for input
	pub fn button(&mut self, label: &str) -> Response {
		let size = vec2(50.0, 20.0);
		let (rect, response) = self.add_rect(size);
		self.ctx.draw_list.add_rect(rect, if response.pressed {self.ctx.colors.button_active} else if response.hovered {self.ctx.colors.button_hovered} else {self.ctx.colors.button});
		response
	}

	/// Creates a rectangle at the cursor with a certain size and returns the rectangle and some input data
	pub fn add_rect(&mut self, size: Vec2) -> (Rect, Response) {
		let rect = Rect::sized(self.cursor, size);
		self.move_cursor(size);
		(rect, self.ctx.check_for_input_at_rect(rect))
	}
}