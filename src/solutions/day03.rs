use regex::Regex;

pub fn part1(input: &Vec<String>) -> i64 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    input
        .iter()
        .flat_map(|line| {
            regex.captures_iter(line).filter_map(|groups| {
                let num1 = groups[1].parse::<i64>().ok()?;
                let num2 = groups[2].parse::<i64>().ok()?;
                Some(num1 * num2)
            })
        })
        .sum()
}

pub fn part2(input: &Vec<String>) -> i64 {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)|(don't\(\))|(do\(\))").unwrap();
    let mut enabled = true;

    // Have to use fold here because we need to keep track of the state of the
    // enabled variable. We can't use map because we can't change the state of
    // the variable, as then we'd have to "move" it into the closure, which
    // would make it impossible to use it in the next iteration.
    //
    // Don't ask me how long it took me to figure this out.
    //
    // Rust is a bit weird with this, I don't know if there's a better way to do
    // this.

    input
        .iter()
        .flat_map(|line| regex.captures_iter(line))
        .fold(0, |mut result: i64, groups| {
            // So, basically, the regex capture will have 4 groups, and their
            // presence will tell us what to do:
            //
            // - If group 1 and 2 are present, we'll have mul(num1,num2) and we
            //   multiply the numbers and add them to the result.
            // - If group 3 is present then we found "don't()", so we disable
            //   the multiplication.
            // - If group 4 is present then we found "do()", so we enable the
            //   multiplication.

            if let (Some(match_num1), Some(match_num2)) = (groups.get(1), groups.get(2)) {
                if enabled {
                    result += match_num1.as_str().parse::<i64>().unwrap() * match_num2.as_str().parse::<i64>().unwrap();
                }
            } else if groups.get(3).is_some() {
                enabled = false;
            } else if groups.get(4).is_some() {
                enabled = true;
            }

            result
        })
}
