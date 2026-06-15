pub fn run() {
	println!("Hello, world!");
}

struct Ranges(Vec<Range>);

impl Ranges {
	fn compact(&mut self) {
		for _range in self.0.iter() {}
	}
}

struct Range {
	lower: u64,
	upper: u64,
}

impl Range {
	fn has(&self, n: u64) -> bool {
		n >= self.lower && n <= self.upper
	}
}
