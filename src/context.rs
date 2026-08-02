use crate::{Colors, drawing::DrawList};
use glam::{Vec2, Vec4};

use crate::Rect;

#[derive(Default)]
pub struct Context {
	colors: Colors,
	draw_list: DrawList
}

