use std::fs::read_to_string;
use std::io::Error;

pub fn get_input_path(day: i32) -> String {
    format!("./inputs/day{:02}.txt", day)
}

pub fn get_example_path(day: i32) -> String {
    format!("./inputs/day{:02}_example.txt", day)
}

pub fn get_input_as_lines(path: &str) -> Result<Vec<String>, Error> {
    read_to_string(path)
        .map(|content| content.lines().map(String::from).collect())
        .map_err(|e| Error::new(e.kind(), format!("Failed to read file '{}': {}", path, e)))
}
