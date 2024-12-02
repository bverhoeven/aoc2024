fn parse_report_lines(lines: &Vec<String>) -> Vec<Vec<i64>> {
    // Each report is a list of numbers called levels that are separated by
    // spaces

    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|level_str| level_str.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe_report(levels: &[i64]) -> bool {
    // A report only counts as safe if both of the following are true:
    // - The levels are either all increasing or all decreasing.
    // - Any two adjacent levels differ by at least one and at most three.

    // Calculate the differences between each level by taking the difference
    // between each pair of adjacent levels.

    let differences: Vec<i64> = levels.windows(2).map(|pair| pair[1] - pair[0]).collect();

    // Check if all differences are either all positive (increasing) or all
    // negative (decreasing) and if all differences are within the range of 1 to
    // 3.

    let all_increasing = differences.iter().all(|&diff| diff >= 0);
    let all_decreasing = differences.iter().all(|&diff| diff <= 0);

    let differences_within_range = differences.iter().all(|&diff| diff.abs() >= 1 && diff.abs() <= 3);

    differences_within_range && (all_decreasing || all_increasing)
}

fn is_safeish_report(levels: &[i64]) -> bool {
    // Same rules apply as before, except if removing a single level from an
    // unsafe report would make it safe, the report instead counts as safe.

    levels.iter().enumerate().any(|(index, _)| {
        let mut modified = levels.to_vec();
        modified.remove(index);
        is_safe_report(&modified)
    })
}

pub fn part1(input: &Vec<String>) -> i64 {
    parse_report_lines(input)
        .iter()
        .filter(|levels| is_safe_report(levels))
        .count() as i64
}

pub fn part2(input: &Vec<String>) -> i64 {
    parse_report_lines(input)
        .iter()
        .filter(|levels| is_safe_report(levels) || is_safeish_report(levels))
        .count() as i64
}

pub fn run(input_lines: Vec<String>) -> (i64, i64) {
    return (part1(&input_lines), part2(&input_lines));
}
