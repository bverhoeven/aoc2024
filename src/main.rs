mod shared;
mod solutions;

use std::env;

use crate::shared::input::{get_example_path, get_input_as_lines, get_input_path};

fn run_day(day: i32, input_lines: Vec<String>) -> (i64, i64) {
    let (part_one, part_two) = match day {
        1 => solutions::day01::run(input_lines),
        2 => solutions::day02::run(input_lines),
        3 => solutions::day03::run(input_lines),
        _ => todo!(),
    };

    (part_one, part_two)
}

fn run_day_with_input_type(day: i32, path: &str, description: &str) -> Result<(), Box<dyn std::error::Error>> {
    let input_lines = get_input_as_lines(path)?;
    let (part_one, part_two) = run_day(day, input_lines);
    println!("{}:", description);
    println!("- Part 1: {}", part_one);
    println!("- Part 2: {}", part_two);
    println!();
    Ok(())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Usage: <day>".into());
    }

    let day = args[1].parse::<i32>().map_err(|_| "Failed to parse day number")?;
    println!("## Day {}\n", day);

    if let Err(e) = run_day_with_input_type(day, &get_example_path(day), "Example input") {
        eprintln!("Example failed: {}", e);
    }

    if let Err(e) = run_day_with_input_type(day, &get_input_path(day), "Puzzle input") {
        eprintln!("Puzzle failed: {}", e);
    }

    Ok(())
}

pub fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
    }
}
