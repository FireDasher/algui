use glam::{Vec2, vec2};

use crate::Rect;

pub struct Vertex {
	pub pos: Vec2,
	pub uv: Vec2,
	pub color: u32,
}
impl Vertex {
	/// Creates a new solid-colored vertex
	#[inline(always)]
	#[must_use]
	pub const fn new(pos: Vec2, color: u32) -> Vertex {
		Vertex { pos, uv: Vec2::ZERO, color }
	}
}

#[derive(Default)]
pub struct DrawList {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u16>,
}

impl DrawList {
	#[inline]
	fn idx_tri(&mut self, a: u16, b: u16, c: u16) {
		self.indices.push(a);
		self.indices.push(b);
		self.indices.push(c);
	}

	/// adds a rectangle\
	pub fn add_rect(&mut self, rect: Rect, color: u32) {
		let offset = self.vertices.len() as u16;
		self.vertices.push(Vertex::new(rect.min, color)); // topleft
		self.vertices.push(Vertex::new(vec2(rect.max.x, rect.min.y), color)); // topright
		self.vertices.push(Vertex::new(vec2(rect.min.x, rect.max.y), color)); // bottomleft
		self.vertices.push(Vertex::new(rect.max, color)); // bottomright
		self.idx_tri(offset, offset + 2, offset + 1);
		self.idx_tri(offset + 1, offset + 2, offset + 3);
	}

	/// adds a triangle on the bottom right, usuaully used for the resizing thing
	pub fn add_bottom_right_triangle(&mut self, rect: Rect, color: u32) {
		let offset = self.vertices.len() as u16;
		self.vertices.push(Vertex::new(vec2(rect.max.x, rect.min.y), color)); // topright
		self.vertices.push(Vertex::new(vec2(rect.min.x, rect.max.y), color)); // bottomleft
		self.vertices.push(Vertex::new(rect.max, color)); // bottomright
		self.idx_tri(offset, offset + 1, offset + 2);
	}
}