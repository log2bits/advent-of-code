use std::fs::read_to_string;

pub fn run() {
    let answer: usize = read_to_string("src/day-3/input.txt")
        .unwrap()
        .lines()
        .map(|x| {
            solve_recursively(x, String::new(), 12)
                .parse::<usize>()
                .unwrap()
        })
        .sum();
    println!("{}", answer);
}

fn solve_recursively(all_digits: &str, mut chosen_digits: String, n_digits: u32) -> String {
    if n_digits == 0 {
        return chosen_digits;
    }
    if all_digits.len() as u32 == n_digits {
        return chosen_digits + all_digits;
    }
    let mut largest_char = char::default();
    let mut largest_n = 0;
    for (n, char) in all_digits[..all_digits.len() - (n_digits - 1) as usize].char_indices() {
        if char > largest_char {
            (largest_char, largest_n) = (char, n);
        }
    }
    let all_digits = &all_digits[(largest_n + 1)..];
    chosen_digits.push(largest_char);

    solve_recursively(all_digits, chosen_digits, n_digits - 1)
}
