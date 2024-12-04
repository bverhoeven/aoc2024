fn count_xmas_occurrences(grid: &Vec<String>) -> i64 {
    let step_directions = [
        (0, 1),  // → Right
        (1, 0),  // ↓ Down
        (1, 1),  // ↘ Sideways (top-left to bottom-right)
        (1, -1), // ↙ Sideways (top-right to bottom-left)
    ];

    let words = ["XMAS", "SAMX"];
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            for &(row_step, col_step) in &step_directions {
                for &word in &words {
                    if grid_has_word(grid, word, r, c, row_step, col_step) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn grid_has_word(
    grid: &Vec<String>,
    word: &str,
    start_row: usize,
    start_col: usize,
    row_step: isize,
    col_step: isize,
) -> bool {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;

    for (i, expected_char) in word.chars().enumerate() {
        let row = start_row as isize + i as isize * row_step;
        if row < 0 || row >= rows {
            return false;
        }

        let col = start_col as isize + i as isize * col_step;
        if col < 0 || col >= cols {
            return false;
        }

        let current_char = grid[row as usize].chars().nth(col as usize);
        if current_char != Some(expected_char) {
            return false;
        }
    }

    true
}

fn count_x_mas_occurrences(grid: &Vec<String>) -> i64 {
    let mut count = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            let current_char = grid[r].chars().nth(c).unwrap();
            if current_char == 'A' {
                let top_left = grid[r - 1].chars().nth(c - 1).unwrap();
                let top_right = grid[r - 1].chars().nth(c + 1).unwrap();
                let bottom_left = grid[r + 1].chars().nth(c - 1).unwrap();
                let bottom_right = grid[r + 1].chars().nth(c + 1).unwrap();

                // ↘ Down right diagonal (top-left to bottom-right)
                //
                // M.. | S..
                // .A. | .A.
                // ..S | ..M

                let is_down_right_diagonal =
                    (top_left == 'M' && bottom_right == 'S') || (top_left == 'S' && bottom_right == 'M');

                // ↙ Down left diagonal (top-right to bottom-left)
                //
                // ..M | ..S
                // .A. | .A.
                // S.. | M..

                let is_down_left_diagonal =
                    (top_right == 'M' && bottom_left == 'S') || (top_right == 'S' && bottom_left == 'M');

                // If both are present, we have a valid X-MAS occurrence, yay!

                if is_down_right_diagonal && is_down_left_diagonal {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn part1(input: &Vec<String>) -> i64 {
    count_xmas_occurrences(input)
}

pub fn part2(input: &Vec<String>) -> i64 {
    count_x_mas_occurrences(input)
}
