use std::collections::HashSet;

pub fn problem_a() -> usize {
    let needed_fields = HashSet::from(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);
    let file = std::fs::read_to_string("res/day4.txt").unwrap();

    file.split("\n\n")
        .filter(|text| {
            let fields: HashSet<_> = text
                .split_whitespace()
                .filter_map(|data| Some(data.split_once(':')?.0))
                .collect();

            fields.is_superset(&needed_fields)
        })
        .count()
}

pub fn problem_b() -> usize {
    let needed_fields = HashSet::from(["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]);
    let file = std::fs::read_to_string("res/day4.txt").unwrap();

    file.split("\n\n")
        .filter(|text| {
            let data: Vec<_> = text
                .split_whitespace()
                .filter_map(|data| data.split_once(':'))
                .collect();

            {
                let fields: HashSet<_> = data.iter().map(|d| d.0).collect();
                if !fields.is_superset(&needed_fields) {
                    return false;
                }
            }

            for (field, value) in data {
                let valid = match field {
                    "byr" => value
                        .parse::<u32>()
                        .map(|v| 1920 <= v && v <= 2002)
                        .unwrap_or(false),
                    "iyr" => value
                        .parse::<u32>()
                        .map(|v| 2010 <= v && v <= 2020)
                        .unwrap_or(false),
                    "eyr" => value
                        .parse::<u32>()
                        .map(|v| 2020 <= v && v <= 2030)
                        .unwrap_or(false),
                    "hgt" => {
                        if value.ends_with("in") {
                            value
                                .trim_end_matches("in")
                                .parse::<u32>()
                                .map(|v| 59 <= v && v <= 76)
                                .unwrap_or(false)
                        } else if value.ends_with("cm") {
                            value
                                .trim_end_matches("cm")
                                .parse::<u32>()
                                .map(|v| 150 <= v && v <= 193)
                                .unwrap_or(false)
                        } else {
                            false
                        }
                    }
                    "hcl" => {
                        (&value[..1] == "#") && value[1..].chars().all(|c| c.is_ascii_hexdigit())
                    }
                    "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
                    "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
                    "cid" => true,
                    _ => unreachable!(),
                };

                if !valid {
                    return false;
                }
            }

            true
        })
        .count()
}
