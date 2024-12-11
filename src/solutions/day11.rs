use cached::proc_macro::cached;

fn parse_input(input: &Vec<String>) -> Vec<u64> {
    input
        .first()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect()
}

#[cached]
fn blink_recursive(stone: u64, rounds: u64) -> u64 {
    match rounds {
        0 => 1,
        _ => match stone {
            0 => blink_recursive(1, rounds - 1),
            _ => match split_stone(stone) {
                Some((left_half, right_half)) => {
                    let left = blink_recursive(left_half, rounds - 1);
                    let right = blink_recursive(right_half, rounds - 1);

                    left + right
                }
                None => blink_recursive(stone * 2024, rounds - 1),
            },
        },
    }
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
    parse_input(input)
        .iter()
        .map(|&stone| blink_recursive(stone, 25))
        .sum::<u64>() as i64
}

pub fn part2(input: &Vec<String>) -> i64 {
    parse_input(input)
        .iter()
        .map(|&stone| blink_recursive(stone, 75))
        .sum::<u64>() as i64
}
