use itertools::Itertools;

fn problem_solution(n: usize) -> usize {
    std::fs::read_to_string("res/day1.txt")
        .unwrap()
        .lines()
        .filter_map(|l| l.parse().ok())
        .combinations(n)
        .find(|x| x.iter().sum::<usize>() == 2020)
        .map(|x| x.iter().product())
        .unwrap()
}

pub fn problem_a() -> usize {
    problem_solution(2)
}

pub fn problem_b() -> usize {
    problem_solution(3)
}
