/// Generic storage based on ImGuiStorage
pub struct Storage<Value> {
	data: Vec<(u32, Value)>
}

impl<Value> Default for Storage<Value> {
	fn default() -> Self {
		Self { data: Vec::new() }
	}
}

impl<Value> Storage<Value> {
	pub fn new() -> Self {
		Self { data: Vec::new() }
	}

	/// Rust reimplementation of ImLowerBound
	fn lower_bound(&self, key: u32) -> usize {
		let mut lo = 0usize;
		let mut count = self.data.len();
		while count > 0 {
			let count2 = count >> 1;
			let mid = lo + count2;
			if self.data[mid].0 < key {
				lo = mid + 1;
				count -= count2 + 1;
			}
			else {
				count = count2;
			}
		}
		lo
	}

	pub fn get(&self, key: u32) -> Option<&Value> {
		let lo = self.lower_bound(key);
		if lo < self.data.len() && self.data[lo].0 == key {
			Some(&self.data[lo].1)
		} else {
			None
		}
	}

	pub fn get_mut(&mut self, key: u32) -> Option<&mut Value> {
		let lo = self.lower_bound(key);
		if lo < self.data.len() && self.data[lo].0 == key {
			Some(&mut self.data[lo].1)
		} else {
			None
		}
	}

	pub fn set(&mut self, key: u32, value: Value) {
		let lo = self.lower_bound(key);
		if lo < self.data.len() && self.data[lo].0 == key {
			self.data[lo].1 = value;
		} else {
			self.data.insert(lo, (key, value));
		}
	}
}

impl<Value: Default + Clone> Storage<Value> {
	pub fn get_or_default(&self, key: u32) -> Value {
		let lo = self.lower_bound(key);
		if lo < self.data.len() && self.data[lo].0 == key {
			self.data[lo].1.clone()
		} else {
			Default::default()
		}
	}
}