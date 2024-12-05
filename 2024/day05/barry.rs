use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/inputs/5"));

    let (ordering_rules, pages) = input.split_once("\n\n").unwrap();

    let pages: Vec<Vec<i64>> = pages
        .lines()
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();
    
    // Store a hashset of all the tuples
    let mut rule_tuples = HashSet::<(i64, i64)>::new();
    for rule in ordering_rules.lines() {
        let (left, right) = rule.split_once('|').unwrap();
        rule_tuples.insert((left.parse().unwrap(), right.parse().unwrap()));
    }

    let mut incorrect_pages = Vec::<Vec<i64>>::new();
    let mut part1 = 0;
    for page in pages {
        let middle_num: i64 = page[page.len() / 2];

        if !does_page_break_rule(&rule_tuples, page.clone()) {
            part1 += middle_num;
        } else {
            incorrect_pages.push(page);
        }
    }

    println!("part1 {}", part1);

    let mut part2 = 0;
    for page in incorrect_pages {
        let fixed_page = get_fixed_ordering(&rule_tuples, page);

        let middle_num: i64 = fixed_page[fixed_page.len() / 2];
        part2 += middle_num;
    }
    println!("part2 {}", part2);
}

fn does_page_break_rule(rule_tuples: &HashSet<(i64, i64)>, page: Vec<i64>) -> bool {
    for (i, l) in page.iter().enumerate() {
        for r in &page[i..] {
            // We only need to check if a rule is broken.
            if rule_tuples.contains(&(*r, *l)) {
                return true;
            }
        }
    }
    false
}

fn get_fixed_ordering(rule_tuples: &HashSet<(i64, i64)>, page: Vec<i64>) -> Vec<i64> {
    let mut new_page = vec![];
    // The count of each occurance of the first in a tuple to the order.
    // The higher the count - it means that number goes first in the ordering.
    let mut number_count = HashMap::<i64, i64>::new();
    for i in 0..page.len() {
        for j in 0..page.len() {
            if i == j {
                continue;
            }
            // In theory only one of these should be non-zero, as if it's incorrect
            // "r" will match, we can't match both.
            let tuplel: (i64, i64) = (page[i], page[j]);
            let tupler: (i64, i64) = (page[j], page[i]);

            if rule_tuples.contains(&tuplel) {
                *number_count.entry(tuplel.0).or_default() += 1;
            }
            if rule_tuples.contains(&tupler) {
                *number_count.entry(tupler.0).or_default() += 1;
            }
        }
    }

    let mut count_vec: Vec<(i64, i64)> = number_count.into_iter().collect();
    count_vec.sort_by(|a, b| a.1.cmp(&b.1));

    for pair in count_vec.iter().rev() {
        new_page.push(pair.0);
    }

    new_page
}
