pub struct Buffer {
	memory: Vec<u8>
}

impl Buffer {
	pub fn new() -> Self {
		Buffer {
			memory: Vec::new(),
		}
	}

	pub fn save(&mut self, buf: &[u8]) {
		self.memory.extend_from_slice(buf);
	}

	pub fn next(&mut self, n: usize, window: &[u8]) -> Option<String> {
		if let Some(split_idx) = self.memory
			.windows(n)
			.position(|w| w == window) {
			let bytes: Vec<u8> = self.memory.drain(..=split_idx + n - 1).collect();
			return Some(String::from_utf8_lossy(&bytes)
				.trim_start_matches('\0')
				.trim()
				.to_string());
		}
		None
	}

	pub fn clear(&mut self) {
		self.memory.clear();
	}

	pub fn is_empty(&self) -> bool {
		self.memory.is_empty()
	}
}

impl Default for Buffer {
	fn default() -> Self {
		Buffer::new()
	}
}