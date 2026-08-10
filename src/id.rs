//! FNV-1a hashes, used internally for storing states

const OFFSET: u32 = 0x811C9DC5;
const PRIME: u32 = 0x01000193;

pub fn id_from_str(string: &str) -> u32 {
	let mut v: u32 = OFFSET;
	for &byte in string.as_bytes() {
		v ^= byte as u32;
		v = v.wrapping_mul(PRIME);
	}
	v
}

pub fn id_from_bytes(bytes: &[u8]) -> u32 {
	let mut v: u32 = OFFSET;
	for &byte in bytes {
		v ^= byte as u32;
		v = v.wrapping_mul(PRIME);
	}
	v
}

pub fn id_from_ref<T>(reference: &T) -> u32 {
	(OFFSET ^ (reference as *const T as u32)).wrapping_mul(PRIME)
}

pub fn id_from_u32(number: u32) -> u32 {
	(OFFSET ^ number).wrapping_mul(PRIME)
}