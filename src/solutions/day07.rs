struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

fn parse_input(lines: &Vec<String>) -> Vec<Equation> {
    lines
        .iter()
        .map(|line| {
            let mut parts = line.split(":");
            Equation {
                test_value: parts.next().unwrap().parse::<i64>().unwrap(),
                numbers: parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect(),
            }
        })
        .collect()
}

fn is_valid_equation(target_value: i64, current_value: i64, remaining_numbers: &[i64], can_concat: bool) -> bool {
    if remaining_numbers.is_empty() {
        return current_value == target_value;
    }

    if current_value > target_value {
        return false;
    }

    let (next, rest) = remaining_numbers.split_first().unwrap();

    let add_value = current_value + next;
    let multiply_value = current_value * next;
    let concat_value: i64 = format!("{}{}", current_value, next).parse().unwrap();

    is_valid_equation(target_value, add_value, rest, can_concat)
        || is_valid_equation(target_value, multiply_value, rest, can_concat)
        || (can_concat && is_valid_equation(target_value, concat_value, rest, can_concat))
}

pub fn part1(input: &Vec<String>) -> i64 {
    let equations = parse_input(input);

    equations
        .iter()
        .filter(|equation| is_valid_equation(equation.test_value, 0, &equation.numbers, false))
        .map(|equation| equation.test_value)
        .sum()
}

pub fn part2(input: &Vec<String>) -> i64 {
    let equations = parse_input(input);

    equations
        .iter()
        .filter(|equation| is_valid_equation(equation.test_value, 0, &equation.numbers, true))
        .map(|equation| equation.test_value)
        .sum()
}
