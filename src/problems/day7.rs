use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct ColoredBag {
    color: String,
    bags_inside: Vec<(String, usize)>,
}

impl ColoredBag {
    pub fn new(text: &str) -> Option<Self> {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"^(\w+ \w+) bags contain (.*)\.$").unwrap();
            static ref BAGS_RE: Regex = Regex::new(r"(\d) (\w+ \w+) bags?").unwrap();
        }

        let line_captures = LINE_RE.captures(text)?;

        let color = line_captures[1].to_string();
        let bags_inside = BAGS_RE
            .captures_iter(&line_captures[2])
            .map(|m| (m[2].to_string(), m[1].parse().unwrap()))
            .collect();

        Some(Self { color, bags_inside })
    }

    pub fn bag_amount(&self, bags: &HashMap<String, Self>) -> usize {
        self.bags_inside
            .iter()
            .map(|(color, amount)| amount * bags[color].bag_amount(&bags))
            .sum::<usize>()
            + 1
    }

    pub fn bags_that_contain<'a>(target: &str, bags: &'a HashMap<String, Self>) -> Vec<&'a str> {
        bags.values()
            .filter(|b| b.bags_inside.iter().find(|(c, _)| c == target).is_some())
            .map(|b| b.color.as_str())
            .collect()
    }
}

pub fn problem_a() -> usize {
    let file = std::fs::read_to_string("res/day7.txt").unwrap();

    let bags: HashMap<String, ColoredBag> = file
        .lines()
        .filter_map(ColoredBag::new)
        .map(|b| (b.color.clone(), b))
        .collect();

    let mut layer = ColoredBag::bags_that_contain("shiny gold", &bags);
    let mut all_possible: HashSet<&str> = layer.iter().cloned().collect();

    while layer.len() > 0 {
        layer = layer
            .iter()
            .flat_map(|c| ColoredBag::bags_that_contain(c, &bags))
            .collect();
        all_possible.extend(&layer);
    }

    all_possible.len()
}

pub fn problem_b() -> usize {
    let file = std::fs::read_to_string("res/day7.txt").unwrap();

    let bags: HashMap<String, ColoredBag> = file
        .lines()
        .filter_map(ColoredBag::new)
        .map(|b| (b.color.clone(), b))
        .collect();

    bags["shiny gold"].bag_amount(&bags) - 1
}
