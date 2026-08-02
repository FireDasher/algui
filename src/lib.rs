/// 32-bit FNV-1a hash
#[inline]
#[must_use]
fn fnv(data: &[u8]) -> u32 {
	let mut v: u32 = 0x811C9DC5;
	for &byte in data {
		v ^= byte as u32;
		v = v.wrapping_mul(0x01000193);
	}
	v
}

#[cfg(feature = "platform")]
pub mod platform;

mod rect;
mod colors;
mod context;
mod ui;
mod storage;
mod drawing;
mod input;

pub use rect::Rect;
pub use colors::Colors;
pub use context::Context;
pub use ui::Ui;
pub use input::Response;

pub mod containers;
pub mod widgets;