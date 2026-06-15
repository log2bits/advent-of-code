use std::fs::read_to_string;

pub fn run() {
    let mut state = State {
        tally: 50,
        password: 0,
    };
    read_to_string("src/day-1/input.txt")
        .unwrap()
        .lines()
        .map(Instruction::parse)
        .for_each(|inst| state.apply(inst));
    println!("{}", state.password);
}

struct State {
    tally: i32,
    password: i32,
}

impl State {
    fn apply(&mut self, inst: Instruction) {
        let new_tally = (self.tally + inst.count).rem_euclid(100);
        let steps = inst.count.abs();
        let needed_steps = if self.tally == 0 {
            100
        } else if inst.count > 0 {
            100 - self.tally
        } else {
            self.tally
        };
        if steps >= needed_steps {
            self.password += 1 + (steps - needed_steps) / 100
        }
        self.tally = new_tally;
    }
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
