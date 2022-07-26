mod problems {
    pub mod day1;
}

fn main() {
    use problems::*;
    for arg in std::env::args().skip(1) {
        let result: String = match arg.to_lowercase().trim_start_matches("day") {
            "1a" => day1::problem_a().into(),
            _ => "unknown problem.".into(),
        };
        println!("{arg}: {result}");
    }
}
