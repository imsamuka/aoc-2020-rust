fn hit_tree(n: usize, line: &str) -> bool {
    line.chars()
        .nth(n % line.len())
        .map(|c| c == '#')
        .unwrap_or_default()
}

pub fn problem_a() -> usize {
    let file = std::fs::read_to_string("res/day3.txt").unwrap();

    file.lines()
        .enumerate()
        .filter(|&(i, l)| hit_tree(i * 3, l))
        .count()
}

pub fn problem_b() -> usize {
    let file = std::fs::read_to_string("res/day3.txt").unwrap();

    file.lines()
        .enumerate()
        .map(|(i, l)| {
            [
                hit_tree(i * 1, l),
                hit_tree(i * 3, l),
                hit_tree(i * 5, l),
                hit_tree(i * 7, l),
                (i % 2 == 0) && hit_tree(i / 2, l),
            ]
            .map(|x| x as usize)
        })
        .reduce(|ac, c| {
            [
                ac[0] + c[0],
                ac[1] + c[1],
                ac[2] + c[2],
                ac[3] + c[3],
                ac[4] + c[4],
            ]
        })
        .unwrap()
        .into_iter()
        .product()
}
