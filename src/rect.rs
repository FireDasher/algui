use glam::{Vec2, vec2};

/// A rectangle.
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Rect {
	pub min: Vec2,
	pub max: Vec2,
}

impl Rect {
	pub const ZERO: Self = Self {
		min: Vec2::ZERO,
		max: Vec2::ZERO,
	};
	pub const ONE: Self = Self {
		min: Vec2::ZERO,
		max: Vec2::ONE,
	};
	pub const EVERYTHING: Self = Self {
		min: Vec2::NEG_INFINITY,
		max: Vec2::INFINITY,
	};
	pub const NAN: Self = Self {
		min: Vec2::NAN,
		max: Vec2::NAN,
	};

	// constructors

	#[inline(always)]
	#[must_use]
	pub const fn new(min: Vec2, max: Vec2) -> Self {
		Self { min, max }
	}

	#[inline]
	#[must_use]
	pub const fn left_of(pos: f32) -> Self {
		Self { min: Vec2::NEG_INFINITY, max: vec2(pos, f32::INFINITY) }
	}
	#[inline]
	#[must_use]
	pub const fn right_of(pos: f32) -> Self {
		Self { min: vec2(pos, f32::NEG_INFINITY), max: Vec2::INFINITY }
	}
	#[inline]
	#[must_use]
	pub const fn above(pos: f32) -> Self {
		Self { min: Vec2::NEG_INFINITY, max: vec2(f32::INFINITY, pos) }
	}
	#[inline]
	#[must_use]
	pub const fn below(pos: f32) -> Self {
		Self { min: vec2(f32::NEG_INFINITY, pos), max: Vec2::INFINITY }
	}

	#[inline]
	#[must_use]
	pub fn centered(center: Vec2, size: Vec2) -> Self {
		Self { min: center - size * 0.5, max: center + size * 0.5 }
	}

	#[inline]
	#[must_use]
	pub fn sized(pos: Vec2, size: Vec2) -> Self {
		Self { min: pos, max: pos + size }
	}

	// getters

	#[inline]
	#[must_use]
	pub fn size(&self) -> Vec2 {
		self.max - self.min
	}

	#[inline]
	#[must_use]
	pub fn width(&self) -> f32 {
		self.max.x - self.min.x
	}

	#[inline]
	#[must_use]
	pub fn height(&self) -> f32 {
		self.max.y - self.min.y
	}

	#[inline]
	#[must_use]
	pub fn area(&self) -> f32 {
		(self.max.x - self.min.x) * (self.max.y - self.min.y)
	}

	// tests

	#[inline]
	#[must_use]
	pub fn contains(&self, point: Vec2) -> bool {
		point.x >= self.min.x && point.y >= self.min.y && point.x <= self.max.x && point.y <= self.max.y
	}

	// operations

	#[inline]
	#[must_use]
	pub fn intersect(&self, other: Rect) -> Rect {
		Rect { min: self.min.max(other.min), max: self.max.min(other.max) }
	}

	#[inline]
	#[must_use]
	pub fn translate(&self, by: Vec2) -> Rect {
		Rect { min: self.min + by, max: self.max + by }
	}

	#[inline]
	#[must_use]
	pub fn shrink(&self, amount: f32) -> Rect {
		Rect { min: self.min + amount, max: self.max - amount }
	}
	#[inline]
	#[must_use]
	pub fn grow(&self, amount: f32) -> Rect {
		Rect { min: self.min - amount, max: self.max + amount }
	}

	#[inline]
	#[must_use]
	pub fn chop_left(&self, pos: f32) -> Rect {
		Rect { min: self.min, max: vec2(pos, self.max.y) }
	}
	#[inline]
	#[must_use]
	pub fn chop_right(&self, pos: f32) -> Rect {
		Rect { min: vec2(pos, self.min.y), max: self.max }
	}
	#[inline]
	#[must_use]
	pub fn chop_above(&self, pos: f32) -> Rect {
		Rect { min: self.min, max: vec2(self.max.x, pos) }
	}
	#[inline]
	#[must_use]
	pub fn chop_below(&self, pos: f32) -> Rect {
		Rect { min: vec2(self.min.x, pos), max: self.max }
	}
}