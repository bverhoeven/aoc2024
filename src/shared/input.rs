use std::fs::read_to_string;
use std::io::Error;
use std::path::Path;

pub fn get_input_path(day: i32) -> String {
    format!("./inputs/day{:02}.txt", day)
}

pub fn get_example_path(day: i32) -> String {
    format!("./inputs/day{:02}_example.txt", day)
}

pub fn get_example_path_for_part(day: i32, part: i32) -> String {
    let part_path = format!("./inputs/day{:02}_example_{}.txt", day, part);
    if Path::new(&part_path).exists() {
        part_path
    } else {
        get_example_path(day)
    }
}

pub fn get_input_as_lines(path: &str) -> Result<Vec<String>, Error> {
    read_to_string(path)
        .map(|content| content.lines().map(String::from).collect())
        .map_err(|e| Error::new(e.kind(), format!("Failed to read file '{}': {}", path, e)))
}
