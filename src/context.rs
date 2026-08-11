use glam::{Vec2, vec2};

use crate::{Colors, Rect, Response, Ui, drawing::DrawList, id::id_from_str, input::Input, storage::Storage};

// Cotnainers
#[derive(Clone, Copy)]
pub struct WindowStorage {
	pub pos: Vec2,
	pub size: Vec2,
	pub collapsed: bool,
}
impl WindowStorage {
	const DEFAULT: Self = Self { pos: vec2(10.0, 10.0), size: vec2(200.0, 500.0), collapsed: false };
}


/// Stores all the global state and stuff
#[derive(Default)]
pub struct Context {
	pub colors: Colors,
	pub draw_list: DrawList,
	pub window_storage: Storage<WindowStorage>,
	pub input: Input,
}

impl Context {
	pub fn check_for_input_at_rect(&self, rect: Rect) -> Response {
		let hovered = rect.contains(self.input.mouse_pos);
		Response { clicked: hovered && self.input.mouse_left_released, pressed: hovered && self.input.mouse_left_down, hovered }
	}

	pub fn window(&mut self, title: &str, add_contents: impl FnOnce(&mut Ui)) {
		let id = id_from_str(&title);
		let storage = self.window_storage.get(id).unwrap_or(&WindowStorage::DEFAULT);

		let separation_position = storage.pos.y + 20.0;

		let title_rect = Rect::new(storage.pos, vec2(storage.pos.x + storage.size.x, separation_position));
		let title_response = self.check_for_input_at_rect(title_rect);
		self.draw_list.add_rect(title_rect, self.colors.title_bg_active);

		let inner_rect = Rect::new(vec2(storage.pos.x, separation_position), storage.pos + storage.size);
		self.draw_list.add_rect(inner_rect, self.colors.window_bg);

		if title_response.pressed {
			// thanks to Rust's borrow checker, we can't just borrow the storage as mutable and change the pos here
			self.window_storage.set(id, WindowStorage { pos: self.input.mouse_pos - title_rect.size() * 0.5, size: storage.size, collapsed: storage.collapsed });
		}

		let mut ui = Ui::new(self, inner_rect.shrink(10.0));
		add_contents(&mut ui);
	}
}