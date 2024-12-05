mod shared;
mod solutions;

use std::env;

use crate::shared::input::get_example_path_for_part;
use crate::shared::input::{get_input_as_lines, get_input_path};
use std::error::Error;

fn run_day(day: i32, input_part1: Vec<String>, input_part2: Vec<String>) -> (i64, i64) {
    match day {
        1 => (
            solutions::day01::part1(&input_part1),
            solutions::day01::part2(&input_part2),
        ),
        2 => (
            solutions::day02::part1(&input_part1),
            solutions::day02::part2(&input_part2),
        ),
        3 => (
            solutions::day03::part1(&input_part1),
            solutions::day03::part2(&input_part2),
        ),
        4 => (
            solutions::day04::part1(&input_part1),
            solutions::day04::part2(&input_part2),
        ),
        5 => (
            solutions::day05::part1(&input_part1),
            solutions::day05::part2(&input_part2),
        ),
        _ => todo!(),
    }
}

fn run_examples(day: i32) -> Result<(), Box<dyn std::error::Error>> {
    let example_one_path = get_example_path_for_part(day, 1);
    let example_two_path = get_example_path_for_part(day, 2);

    let example_part_one_input = get_input_as_lines(&example_one_path)?;
    let example_part_two_input = get_input_as_lines(&example_two_path)?;

    let (example_part_one, example_part_two) = run_day(day, example_part_one_input, example_part_two_input);

    println!("Example:");
    println!("- Part 1 ({}): {}", example_one_path, example_part_one);
    println!("- Part 2 ({}): {}", example_two_path, example_part_two);
    println!();

    Ok(())
}

fn run_puzzle(day: i32) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = get_input_path(day);
    let input_part_one = get_input_as_lines(&input_path)?;
    let input_part_two = input_part_one.clone();

    let (part_one, part_two) = run_day(day, input_part_one, input_part_two);

    println!("Puzzle:");
    println!("- Part 1: {}", part_one);
    println!("- Part 2: {}", part_two);

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: <day>".into());
    }

    let day: i32 = args[1].parse().map_err(|_| "Failed to parse day number")?;

    println!("## Day {}\n", day);
    run_examples(day)?;
    run_puzzle(day)?;

    Ok(())
}

pub fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
