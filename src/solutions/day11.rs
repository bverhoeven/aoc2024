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
fn blink(stone: u64, rounds: u64) -> u64 {
    if rounds == 0 {
        return 1;
    }

    let rounds_remaining = rounds - 1;
    match stone {
        0 => blink(1, rounds_remaining),
        _ => match split_stone(stone) {
            Some((left_half, right_half)) => blink(left_half, rounds_remaining) + blink(right_half, rounds_remaining),
            None => blink(stone * 2024, rounds_remaining),
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
    parse_input(input).iter().map(|&stone| blink(stone, 25)).sum::<u64>() as i64
}

pub fn part2(input: &Vec<String>) -> i64 {
    parse_input(input).iter().map(|&stone| blink(stone, 75)).sum::<u64>() as i64
}
