mod problems {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
    pub mod day5;
    pub mod day6;
}

fn main() {
    use problems::*;
    for arg in std::env::args().skip(1) {
        let result = match arg.to_lowercase().trim_start_matches("day") {
            "1a" => day1::problem_a(),
            "1b" => day1::problem_b(),
            "2a" => day2::problem_a(),
            "2b" => day2::problem_b(),
            "3a" => day3::problem_a(),
            "3b" => day3::problem_b(),
            "4a" => day4::problem_a(),
            "4b" => day4::problem_b(),
            "5a" => day5::problem_a(),
            "5b" => day5::problem_b(),
            "6a" => day6::problem_a(),
            "6b" => day6::problem_b(),
            _ => 0,
        };
        println!("{arg}: {result}");
    }
}
