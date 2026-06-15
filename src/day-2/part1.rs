use std::collections::HashSet;
use std::fs::read_to_string;

pub fn run() {
    let mut unique = HashSet::new();
    read_to_string("src/day-2/input.txt")
        .unwrap()
        .split(',')
        .map(IDs::parse)
        .for_each(|ids| unique.extend(ids.find_invalid().iter().copied()));
    let total: usize = unique.iter().sum();
    println!("{}", total);
}

struct IDs {
    start: String,
    end: String,
}

impl IDs {
    fn parse(string: &str) -> Self {
        let (start, end) = string.split_once('-').unwrap();
        Self {
            start: String::from(start.trim()),
            end: String::from(end.trim()),
        }
    }

    fn find_invalid(&self) -> Vec<usize> {
        let mut invalid: Vec<usize> = Vec::new();
        let max_num: usize = self.end.parse().unwrap();
        let min_num: usize = self.start.parse().unwrap();
        // Only consider sequences repeated exactly twice.
        for repeated_slice in 0..10usize.pow(self.end.len() as u32 / 2) {
            let current_num_str = repeated_slice.to_string().repeat(2);
            let current_num: usize = current_num_str.parse().unwrap();
            if current_num >= min_num && current_num <= max_num {
                invalid.push(current_num);
            }
        }
        invalid
    }
}
