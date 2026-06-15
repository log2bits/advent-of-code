use std::collections::HashSet;
use std::fs::read_to_string;

pub fn run() {
    let mut state = State { hits: 0, misses: 0 };
    let mut unique = HashSet::new();
    read_to_string("src/day-2/input.txt")
        .unwrap()
        .split(',')
        .map(IDs::parse)
        .for_each(|ids| unique.extend(ids.find_invalid(&mut state).iter().copied()));
    let total: usize = unique.iter().sum();
    println!("{}", total);
    println!("hits: {}", state.hits);
    println!("misses: {}", state.misses);
}

struct IDs {
    start: String,
    end: String,
}

struct State {
    pub hits: u64,
    pub misses: u64,
}

impl IDs {
    fn parse(string: &str) -> Self {
        let (start, end) = string.split_once('-').unwrap();
        Self {
            start: String::from(start.trim()),
            end: String::from(end.trim()),
        }
    }

    fn find_invalid(&self, state: &mut State) -> Vec<usize> {
        let mut invalid: Vec<usize> = Vec::new();
        let max_num = self.end.parse().unwrap();
        let min_num = self.start.parse().unwrap();
        for repeated_slice in 0..10usize.pow(self.end.len() as u32 / 2) {
            let slice_len = repeated_slice.to_string().len();
            let min_repeat = (self.start.len() / slice_len).max(2);
            let max_repeat = (self.end.len() / slice_len) + 1;
            for repeat in min_repeat..max_repeat {
                let current_num = repeated_slice.to_string().repeat(repeat).parse().unwrap();
                if current_num >= min_num && current_num <= max_num {
                    invalid.push(current_num);
                    state.hits += 1;
                } else {
                    state.misses += 1;
                }
            }
        }
        invalid
    }
}
