use std::cmp::Ordering;

fn parse_input(lines: &Vec<String>) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let mut updates: Vec<Vec<i64>> = Vec::new();
    let mut ordering_rules: Vec<(i64, i64)> = Vec::new();

    // This is a little bit hacky, but I didn't want to create two separate
    // loops for the ordering rules and the updates, and the input allows us to
    // easily determine which is which.

    lines.iter().for_each(|line| {
        if line.contains("|") {
            // Page ordering rule. Example: 47|53
            //
            // If an update includes both page number 47 and page number 53,
            // then page number 47 must be printed at some point before page
            // number 53.

            let mut parts = line.split("|");
            ordering_rules.push((
                parts.next().unwrap().parse::<i64>().unwrap(),
                parts.next().unwrap().parse::<i64>().unwrap(),
            ));
        }

        if line.contains(",") {
            // Page numbers of each update. Example: 75,47,61,53,29 Parse the
            // page numbers and add them to the updates list.

            updates.push(line.split(",").map(|page| page.parse::<i64>().unwrap()).collect());
        }
    });

    (ordering_rules, updates)
}

fn sort_pages(pages: &Vec<i64>, ordering: &Vec<(i64, i64)>) -> Vec<i64> {
    let mut sorted = pages.to_vec();

    sorted.sort_by(|a, b| {
        if ordering.iter().any(|(before, after)| a == after && b == before) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    sorted
}

pub fn get_middle_page(sorted: &Vec<i64>) -> i64 {
    sorted[sorted.len() / 2]
}

pub fn part1(input: &Vec<String>) -> i64 {
    let (ordering, updates) = parse_input(&input);
    let mut result = 0;

    for pages_in_update in &updates {
        let sorted_pages = sort_pages(pages_in_update, &ordering);
        if pages_in_update == &sorted_pages {
            result += get_middle_page(&sorted_pages);
        }
    }

    result
}

pub fn part2(input: &Vec<String>) -> i64 {
    let (ordering, updates) = parse_input(&input);
    let mut result = 0;

    for pages_in_update in &updates {
        let sorted_pages = sort_pages(pages_in_update, &ordering);
        if pages_in_update != &sorted_pages {
            result += get_middle_page(&sorted_pages);
        }
    }

    result
}
