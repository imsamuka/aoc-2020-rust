use std::collections::HashSet;

pub fn problem_a() -> usize {
    let file = std::fs::read_to_string("res/day6.txt").unwrap();

    file.split("\n\n")
        .map(|text| {
            text.chars()
                .filter(|c| c.is_ascii_alphabetic())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

pub fn problem_b() -> usize {
    let file = std::fs::read_to_string("res/day6.txt").unwrap();

    file.split("\n\n")
        .map(|text| {
            text.split_whitespace()
                .map(|l| l.chars().collect::<HashSet<char>>())
                .reduce(|ac, other| ac.intersection(&other).cloned().collect())
                .map(|result| result.len())
                .unwrap_or_default()
        })
        .sum()
}
