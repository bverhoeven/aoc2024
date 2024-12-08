use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn is_in_map_bounds(coordinate: Coordinate, map_bounds: i32) -> bool {
    coordinate.x >= 0 && coordinate.y >= 0 && coordinate.x < map_bounds && coordinate.y < map_bounds
}

fn place_antinodes(coordinates: &[Coordinate], antinodes: &mut HashSet<Coordinate>, map_bounds: i32) {
    for (index, &current) in coordinates.iter().enumerate() {
        for &other in &coordinates[index + 1..] {
            let distance = other - current;

            let first = current - distance;
            if is_in_map_bounds(first, map_bounds) {
                antinodes.insert(first);
            }

            let second = other + distance;
            if is_in_map_bounds(second, map_bounds) {
                antinodes.insert(second);
            }
        }
    }
}

fn place_loads_of_antinodes(coordinates: &[Coordinate], antinodes: &mut HashSet<Coordinate>, map_bounds: i32) {
    for (index, &current) in coordinates.iter().enumerate() {
        for &other in &coordinates[index + 1..] {
            let distance = other - current;

            let mut first = current - distance;
            while is_in_map_bounds(first, map_bounds) {
                antinodes.insert(first);
                first = first - distance;
            }

            let mut second = other + distance;
            while is_in_map_bounds(second, map_bounds) {
                antinodes.insert(second);
                second = second + distance;
            }
        }
    }
}

pub fn part1(input: &Vec<String>) -> i64 {
    let mut freq_antennas: HashMap<char, Vec<Coordinate>> = HashMap::new();
    let mut antinodes: HashSet<Coordinate> = HashSet::new();

    assert!(input.len() == input[0].len());

    for (x, line) in input.iter().enumerate() {
        for (y, frequency) in line.chars().enumerate().filter(|&(_, c)| c != '.') {
            freq_antennas.entry(frequency).or_default().push(Coordinate {
                x: x as i32,
                y: y as i32,
            });
        }
    }

    let map_bounds = input.len() as i32;
    for coordinates in freq_antennas.values() {
        place_antinodes(coordinates, &mut antinodes, map_bounds);
    }

    antinodes.len() as i64
}

pub fn part2(input: &Vec<String>) -> i64 {
    let mut freq_antennas: HashMap<char, Vec<Coordinate>> = HashMap::new();
    let mut antinodes: HashSet<Coordinate> = HashSet::new();

    for (x, line) in input.iter().enumerate() {
        for (y, frequency) in line.chars().enumerate().filter(|&(_, c)| c != '.') {
            let coordinate = Coordinate {
                x: x as i32,
                y: y as i32,
            };

            freq_antennas.entry(frequency).or_default().push(coordinate);
            antinodes.insert(coordinate);
        }
    }

    let map_bounds = input.len() as i32;
    for coordinates in freq_antennas.values() {
        place_loads_of_antinodes(coordinates, &mut antinodes, map_bounds);
    }

    antinodes.len() as i64
}
