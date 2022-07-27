use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

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

    pub fn bag_amount(&self, bags: &[Self]) -> usize {
        self.bags_inside
            .iter()
            .map(|(color, amount)| amount * ColoredBag::find_bag(&color, &bags).bag_amount(&bags))
            .sum::<usize>()
            + 1
    }

    pub fn find_bag<'a>(target: &str, bags: &'a [Self]) -> &'a ColoredBag {
        bags.iter().find(|b| b.color == target).unwrap()
        // ! really insecure, but fine for the challenge
    }

    pub fn bags_that_contain<'a>(target: &str, bags: &'a [Self]) -> Vec<&'a str> {
        bags.iter()
            .filter(|b| b.bags_inside.iter().find(|(c, _)| c == target).is_some())
            .map(|b| b.color.as_str())
            .collect()
    }
}

pub fn problem_a() -> usize {
    let file = std::fs::read_to_string("res/day7.txt").unwrap();

    let bags: Vec<_> = file.lines().filter_map(ColoredBag::new).collect();

    let mut layer = ColoredBag::bags_that_contain("shiny gold", &bags);
    let mut all_possible: HashSet<&str> = layer.iter().cloned().collect();

    while layer.len() > 0 {
        layer = layer
            .iter()
            .flat_map(|c| ColoredBag::bags_that_contain(c, &bags))
            .collect();
        all_possible.extend(layer.iter());
    }

    all_possible.len()
}

pub fn problem_b() -> usize {
    let file = std::fs::read_to_string("res/day7.txt").unwrap();
    let bags: Vec<_> = file.lines().filter_map(ColoredBag::new).collect();
    ColoredBag::find_bag("shiny gold", &bags).bag_amount(&bags) - 1
}
