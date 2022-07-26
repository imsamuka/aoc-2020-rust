pub fn problem_a() -> usize {
    let re = regex::Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let file = std::fs::read_to_string("res/day2.txt").unwrap();

    file.lines()
        .filter_map(|l| re.captures(l))
        .filter(|cap| {
            let min_bound: usize = cap[1].parse().unwrap();
            let max_bound: usize = cap[2].parse().unwrap();
            let chr = cap[3].chars().next().unwrap();
            let password = &cap[4];

            let amount = password.chars().filter(|&c| c == chr).count();
            min_bound <= amount && amount <= max_bound
        })
        .count()
}

pub fn problem_b() -> usize {
    let re = regex::Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let file = std::fs::read_to_string("res/day2.txt").unwrap();
    file.lines()
        .filter_map(|l| re.captures(l))
        .filter(|cap| {
            let index_1 = cap[1].parse::<usize>().unwrap() - 1;
            let index_2 = cap[2].parse::<usize>().unwrap() - 1;
            let chr = cap[3].chars().next().unwrap();
            let password = &cap[4];

            let in_index_1 = password
                .chars()
                .nth(index_1)
                .map(|c| c == chr)
                .unwrap_or_default();
            let in_index_2 = password
                .chars()
                .nth(index_2)
                .map(|c| c == chr)
                .unwrap_or_default();

            in_index_1 ^ in_index_2
        })
        .count()
}
