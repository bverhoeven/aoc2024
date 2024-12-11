use std::collections::HashMap;

fn parse_input(input: &Vec<String>) -> Vec<u64> {
    input
        .first()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect()
}

fn blink_recursive(stone: u64, rounds: u64, cache: &mut HashMap<(u64, u64), u64>) -> u64 {
    cache.get(&(stone, rounds)).copied().unwrap_or_else(|| {
        let stone_count = match rounds {
            0 => 1,
            _ => match stone {
                0 => blink_recursive(1, rounds - 1, cache),
                _ => match split_stone(stone) {
                    Some((left_half, right_half)) => {
                        let left = blink_recursive(left_half, rounds - 1, cache);
                        let right = blink_recursive(right_half, rounds - 1, cache);

                        left + right
                    }
                    None => blink_recursive(stone * 2024, rounds - 1, cache),
                },
            },
        };

        cache.insert((stone, rounds), stone_count);
        stone_count
    })
}

fn split_stone(engraving: u64) -> Option<(u64, u64)> {
    let engraving_str = engraving.to_string();
    if engraving_str.len() % 2 != 0 {
        return None;
    }

    let middle = engraving_str.len() / 2;
    let (left, right) = engraving_str.split_at(middle);

    Some((left.parse().unwrap(), right.parse().unwrap()))
}

pub fn part1(input: &Vec<String>) -> i64 {
    let cache: &mut HashMap<(u64, u64), u64> = &mut HashMap::new();

    parse_input(input)
        .iter()
        .map(|&stone| blink_recursive(stone, 25, cache))
        .sum::<u64>() as i64
}

pub fn part2(input: &Vec<String>) -> i64 {
    let cache: &mut HashMap<(u64, u64), u64> = &mut HashMap::new();

    parse_input(input)
        .iter()
        .map(|&stone_number| blink_recursive(stone_number, 75, cache))
        .sum::<u64>() as i64
}
