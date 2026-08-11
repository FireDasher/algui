use glam::Vec2;

#[derive(Default, Clone, Copy)]
pub struct Response {
	pub clicked: bool,
	pub pressed: bool,
	pub hovered: bool,
}

#[derive(Default, Clone, Copy)]
pub struct Input {
	pub mouse_pos: Vec2,

	pub mouse_left_clicked: bool,
	pub mouse_left_down: bool,
	pub mouse_left_released: bool,

	pub mouse_right_clicked: bool,
	pub mouse_right_down: bool,
	pub mouse_right_released: bool,
}