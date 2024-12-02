fn parse_and_sort_pairs(lines: &Vec<String>) -> (Vec<i64>, Vec<i64>) {
    let mut first_values: Vec<i64> = Vec::new();
    let mut second_values: Vec<i64> = Vec::new();

    lines.iter().for_each(|line| {
        let mut parts = line.split_whitespace();
        let first = parts.next().unwrap().parse::<i64>().unwrap();
        let second = parts.next().unwrap().parse::<i64>().unwrap();

        first_values.push(first);
        second_values.push(second);
    });

    first_values.sort();
    second_values.sort();

    (first_values, second_values)
}

pub fn part1(input: &Vec<String>) -> i64 {
    let (first_values, second_values) = parse_and_sort_pairs(input);

    first_values
        .iter()
        .zip(second_values.iter())
        .fold(0, |total, (&first, &second)| total + (first - second).abs())
}

pub fn part2(input: &Vec<String>) -> i64 {
    let (first_values, second_values) = parse_and_sort_pairs(input);

    first_values.iter().fold(0, |total, &first| {
        total + first * second_values.iter().filter(|&&second| second == first).count() as i64
    })
}

pub fn run(input_lines: Vec<String>) -> (i64, i64) {
    return (part1(&input_lines), part2(&input_lines));
}
