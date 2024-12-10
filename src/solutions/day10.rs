use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    pub fn from<T: TryInto<i32>, U: TryInto<i32>>(x: T, y: U) -> Self
    where
        T::Error: std::fmt::Debug,
        U::Error: std::fmt::Debug,
    {
        Self {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        }
    }

    fn neighbors(&self, boundary: i32) -> Vec<Coordinate> {
        vec![
            Coordinate::from(self.x - 1, self.y),
            Coordinate::from(self.x, self.y + 1),
            Coordinate::from(self.x + 1, self.y),
            Coordinate::from(self.x, self.y - 1),
        ]
        .into_iter()
        .filter(|coord| coord.x >= 0 && coord.y >= 0 && coord.x < boundary && coord.y < boundary)
        .collect()
    }
}

#[derive(Debug, Clone)]
struct TrailMap {
    map: Vec<Vec<u8>>,
    start_locations: Vec<Coordinate>,
}

impl TrailMap {
    pub fn from_input(input: &Vec<String>) -> Self {
        let mut start_locations: Vec<Coordinate> = vec![];
        let mut map: Vec<Vec<u8>> = vec![];

        for (x, line) in input.iter().enumerate() {
            let mut row: Vec<u8> = vec![];
            for (y, score_char) in line.chars().enumerate() {
                let score = score_char.to_digit(10).unwrap() as u8;
                if score == 0 {
                    start_locations.push(Coordinate::from(x, y))
                }

                row.push(score);
            }

            map.push(row);
        }

        Self { map, start_locations }
    }

    fn size(&self) -> i32 {
        self.map.len() as i32
    }

    fn score_at(&self, coordinate: &Coordinate) -> u8 {
        self.map[coordinate.x as usize][coordinate.y as usize]
    }
}

fn solve(start: Coordinate, map: &TrailMap, visit_all: bool) -> u32 {
    let mut to_visit = VecDeque::from([start]);
    let mut visited = HashSet::from([start]);
    let mut count = 0;

    let map_size = map.size();

    while let Some(coordinate) = to_visit.pop_front() {
        let score = map.score_at(&coordinate);
        if score == 9 {
            count += 1;
            continue;
        }

        for neighbor_coordinate in coordinate.neighbors(map_size) {
            if visit_all || !visited.contains(&neighbor_coordinate) {
                if map.score_at(&neighbor_coordinate) == score + 1 {
                    visited.insert(neighbor_coordinate);
                    to_visit.push_back(neighbor_coordinate);
                }
            }
        }
    }

    count
}

pub fn part1(input: &Vec<String>) -> i64 {
    let map = TrailMap::from_input(input);

    map.start_locations
        .iter()
        .map(|&start| solve(start, &map, false))
        .sum::<u32>() as i64
}

pub fn part2(input: &Vec<String>) -> i64 {
    let map = TrailMap::from_input(input);

    map.start_locations
        .iter()
        .map(|&start| solve(start, &map, true))
        .sum::<u32>() as i64
}
