use crate::{Colors, containers::WindowStorage, drawing::DrawList, storage::Storage};
use glam::{Vec2, Vec4};

use crate::Rect;

#[derive(Default)]
pub struct Context {
	pub colors: Colors,
	pub draw_list: DrawList,
	pub window_storage: Storage<WindowStorage>,
}

