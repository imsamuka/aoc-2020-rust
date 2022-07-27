mod problems {
    pub mod day1;
    pub mod day2;
    pub mod day3;
    pub mod day4;
}

fn main() {
    use problems::*;
    for arg in std::env::args().skip(1) {
        let result: String = match arg.to_lowercase().trim_start_matches("day") {
            "1a" => day1::problem_a().to_string(),
            "1b" => day1::problem_b().to_string(),
            "2a" => day2::problem_a().to_string(),
            "2b" => day2::problem_b().to_string(),
            "3a" => day3::problem_a().to_string(),
            "3b" => day3::problem_b().to_string(),
            "4a" => day4::problem_a().to_string(),
            "4b" => day4::problem_b().to_string(),
            _ => "unknown problem.".into(),
        };
        println!("{arg}: {result}");
    }
}

// pub fn read_file(filename: &str) {}
