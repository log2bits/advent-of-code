use std::fs::read_to_string;

pub fn run() {
	let mut tally: i32 = 50;
	let mut password = 0;
	read_to_string("src/day-1/input.txt")
		.unwrap()
		.lines()
		.map(Instruction::parse)
		.for_each(|inst| {
			tally = (tally + inst.count).rem_euclid(100);
			if tally == 0 {
				password += 1;
			}
		});
	println!("{}", password);
}

struct Instruction {
	count: i32,
}

impl Instruction {
	fn parse(string: &str) -> Instruction {
		let (sign, digits) = string.split_at(1);
		let n: i32 = digits.parse().unwrap_or(0);
		Instruction::new(match sign {
			"L" => -n,
			"R" => n,
			_ => 0,
		})
	}

	fn new(count: i32) -> Self {
		Self { count }
	}
}
