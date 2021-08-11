pub struct Stack {
	content: Vec<i64>
}

// numbers only
impl Stack {
	pub fn new() -> Stack {
		Stack {
			content: Vec::new()
		}
	}

	pub fn push(&mut self, to_push: i64) {
		self.push(to_push);
	}

	pub fn pop(&mut self) -> i64 {
		let result = self.content.pop();
		
		match result {
			None => panic!("Error, not enough in stack to pop"),
			Some(s) => s
		}
	}
}
