use crate::{Colors, drawing::DrawList};
use glam::{Vec2, Vec4};

use crate::Rect;

#[derive(Default)]
pub struct Context {
	pub colors: Colors,
	pub draw_list: DrawList
}

