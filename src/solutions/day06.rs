use std::collections::HashSet;
use std::thread;

#[derive(Clone, Debug, PartialEq)]
enum Entity {
    None,
    Guard,
    Obstacle,
}

const DIRECTIONS: [(i64, i64); 4] = [
    /* up    */ (-1, 0),
    /* right */ (0, 1),
    /* down  */ (1, 0),
    /* left  */ (0, -1),
];

fn parse_input(input: &Vec<String>) -> Vec<Vec<Entity>> {
    input
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Entity::None,
                    '^' => Entity::Guard,
                    '#' => Entity::Obstacle,
                    _ => panic!("Invalid entity type"),
                })
                .collect()
        })
        .collect()
}

fn find_guard_coordinates(grid: &Vec<Vec<Entity>>) -> (i64, i64) {
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, entity) in row.iter().enumerate() {
            if entity == &Entity::Guard {
                return (row_index as i64, col_index as i64);
            }
        }
    }

    panic!("Guard not found");
}

fn next_position(grid: &[Vec<Entity>], x: i64, y: i64, (direction_x, direction_y): (i64, i64)) -> Option<(i64, i64)> {
    let (next_x, next_y) = (x + direction_x, y + direction_y);
    if is_in_grid_bounds(grid, next_x, next_y) {
        Some((next_x, next_y))
    } else {
        None
    }
}

fn is_in_grid_bounds(grid: &[Vec<Entity>], x: i64, y: i64) -> bool {
    x >= 0 && y >= 0 && x < grid.len() as i64 && y < grid[0].len() as i64
}

fn is_obstacle(grid: &[Vec<Entity>], x: i64, y: i64) -> bool {
    grid[x as usize][y as usize] == Entity::Obstacle
}

fn is_loop(grid: &[Vec<Entity>], (mut x, mut y): (i64, i64)) -> bool {
    let mut visited = HashSet::new();
    let mut direction_index = 0;

    loop {
        let key = (x, y, direction_index);
        if !visited.insert(key) {
            return true; // Loop detected
        }

        if let Some((next_x, next_y)) = next_position(grid, x, y, DIRECTIONS[direction_index]) {
            // When we hit an obstacle, we need to change direction, just like in
            // part 1.

            if is_obstacle(grid, next_x, next_y) {
                direction_index = (direction_index + 1) % DIRECTIONS.len();
            } else {
                x = next_x;
                y = next_y;
            }
        } else {
            // The next position is out of bounds, so we're done. Placing the
            // obstacle did not create a loop.

            return false;
        }
    }
}

fn count_loops_for_row(grid: &[Vec<Entity>], start_pos: (i64, i64), x: usize) -> i64 {
    grid[x]
        .iter()
        .enumerate()
        .filter(|(_, &ref cell)| cell == &Entity::None)
        .filter_map(|(y, _)| {
            let mut local_grid = grid.to_vec();
            local_grid[x][y] = Entity::Obstacle;
            is_loop(&local_grid, start_pos).then_some(1)
        })
        .sum()
}

pub fn part1(input: &Vec<String>) -> i64 {
    let grid = parse_input(input);
    let (mut x, mut y) = find_guard_coordinates(&grid);

    let mut direction_index = 0;
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    visited[x as usize][y as usize] = true;

    while let Some((next_x, next_y)) = next_position(&grid, x, y, DIRECTIONS[direction_index]) {
        if is_obstacle(&grid, next_x, next_y) {
            // Move clockwise. Up -> Right -> Down -> Left -> Up -> ...

            direction_index = (direction_index + 1) % DIRECTIONS.len();
        } else {
            // Continue moving and mark the cell as visited.

            x = next_x;
            y = next_y;

            visited[x as usize][y as usize] = true;
        }
    }

    visited
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|&tile| tile)
        .count() as i64
}

pub fn part2(input: &Vec<String>) -> i64 {
    part2_threads(input)
}

/**
 * Faster, multi-threaded solution.
 *
 * Probably not very Rust, but eh, it works. Yay for parallelism!
 *
 * Takes less than a second when compiled with `--release` and
 * about 5-7 seconds without it (for both the example and the puzzle input).
 */
pub fn part2_threads(input: &Vec<String>) -> i64 {
    let grid = parse_input(input);
    let start_pos = find_guard_coordinates(&grid);

    let mut threads = vec![];
    for row_index in 0..grid.len() {
        let grid_clone = grid.clone();
        threads.push(thread::spawn(move || {
            count_loops_for_row(&grid_clone, start_pos, row_index)
        }));
    }

    let mut total_loops = 0;
    for handle in threads {
        total_loops += handle.join().unwrap();
    }

    total_loops
}

/**
 * Naive, single-threaded solution.
 *
 * Takes about 10 seconds when compiled with `--release`, but
 * well over 1 minute in unoptimized debug builds.
 *
 * Keeping it here for reference :D
 */
#[allow(dead_code)]
pub fn part2_slow(input: &Vec<String>) -> i64 {
    let grid = parse_input(input);
    let start_pos = find_guard_coordinates(&grid);

    let mut loops = 0;
    for row_index in 0..grid.len() {
        loops += count_loops_for_row(&grid, start_pos, row_index);
    }

    loops
}
