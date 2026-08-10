use glam::{Vec2, vec2};

use crate::{Context, Rect, Ui, id::id_from_str};

#[derive(Clone, Copy)]
pub struct WindowStorage {
	pub pos: Vec2,
	pub size: Vec2,
	pub collapsed: bool,
}

impl Default for WindowStorage {
	fn default() -> Self {
		Self { pos: vec2(10.0, 10.0), size: vec2(200.0, 500.0), collapsed: false }
	}
}

pub struct Window {
	title: String,
}

impl Window {
	pub fn new(title: &str) -> Self {
		Window { title: title.to_string() }
	}

	/// Displays the window and handles input
	pub fn show(&self, ctx: &mut Context, add_contents: impl FnOnce(&mut Ui)) {
		let id = id_from_str(&self.title);
		let storage = ctx.window_storage.get_or_default(id);

		let title_rect = Rect::new(storage.pos, vec2(storage.pos.x + storage.size.x, storage.pos.y + 10.0));
		ctx.draw_list.add_rect(title_rect, ctx.colors.title_bg);

		let inner_rect = Rect::new(vec2(storage.pos.x, storage.pos.y + 10.0), storage.pos + storage.size);
		ctx.draw_list.add_rect(inner_rect, ctx.colors.window_bg);

		let mut ui = Ui { ctx, rect: inner_rect, cursor: inner_rect.min + 10.0, horizontal: false };
		add_contents(&mut ui);
	}
}