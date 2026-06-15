use std::fs::read_to_string;

pub fn run() {
    let text = read_to_string("src/day-4/input.txt").unwrap();
    let grid = text_to_array(text);
    let solution = solve(&grid);
    println!("{}", solution);
}

fn text_to_array(text: String) -> Vec<Vec<bool>> {
    text.lines()
        .map(|line| line.chars().map(|c| c == '@').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>()
}

fn solve(grid: &Vec<Vec<bool>>) -> u32 {
    let mut valid_cells = 0;
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] && count_adjacent(grid, row, col) < 4 {
                valid_cells += 1;
            }
        }
    }
    valid_cells
}

fn count_adjacent(grid: &Vec<Vec<bool>>, col: usize, row: usize) -> i32 {
    let mut count = -1;
    for row_offset in -1..=1 {
        for col_offset in -1..=1 {
            let r = (row as i32 + row_offset) as usize;
            let c = (col as i32 + col_offset) as usize;

            if let Some(&cell) = grid.get(r).and_then(|row| row.get(c)) {
                count += cell as i32;
            }
        }
    }
    count
}
