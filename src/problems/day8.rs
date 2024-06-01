#[derive(Debug, Clone, Copy)]
enum OP {
    ACC,
    JMP,
    NOP,
}

#[derive(Debug)]
struct Inst(OP, i32);

fn extract_operations(file: &str) -> Vec<Inst> {
    file.lines()
        .map(str::split_ascii_whitespace)
        .map(Iterator::collect::<Vec<&str>>)
        .enumerate()
        .map(|(index, v)| {
            let &[op, value] = v.as_slice() else {
                panic!("Invalid operation in line {}", index);
            };

            Inst(
                match op {
                    "nop" => OP::NOP,
                    "acc" => OP::ACC,
                    "jmp" => OP::JMP,
                    op => panic!("Invalid operation \"{}\"", op),
                },
                value.parse().unwrap(),
            )
        })
        .collect()
}

fn get_last_acc(instructions: &[Inst]) -> (i32, usize) {
    let mut acc = 0;
    let mut index = 0;
    let mut accessed = Vec::with_capacity(instructions.len());

    while let Err(insertion_index) = accessed.binary_search(&index) {
        accessed.insert(insertion_index, index);

        if index == instructions.len() {
            break;
        }

        let Inst(op, value) = instructions[index];

        match op {
            OP::NOP => index += 1,
            OP::ACC => {
                acc += value;
                index += 1;
            }
            OP::JMP => index = (index as i32 + value) as usize,
        };
    }

    (acc, index)
}

pub fn problem_a() -> usize {
    let file = std::fs::read_to_string("res/day8.txt").unwrap();
    let instructions = extract_operations(&file);
    drop(file);
    get_last_acc(&instructions).0 as usize
}

pub fn problem_b() -> usize {
    let file = std::fs::read_to_string("res/day8.txt").unwrap();
    let mut instructions = extract_operations(&file);
    drop(file);

    let nops = instructions
        .iter()
        .enumerate()
        .filter(|(_, &Inst(op, v))| matches!(op, OP::NOP) && v != 0 && v != 1)
        .filter(|(i, &Inst(_, v))| {
            let j = *i as i32 + v;
            !(j < 0 || j as usize >= instructions.len())
        })
        .map(|e| e.0)
        .collect::<Vec<_>>();

    let jmps = instructions
        .iter()
        .enumerate()
        .filter(|(_, &Inst(op, v))| matches!(op, OP::JMP) && v != 1)
        .map(|e| e.0)
        .collect::<Vec<_>>();

    for i in jmps.into_iter().chain(nops) {
        let op = instructions[i].0;
        instructions[i].0 = match op {
            OP::JMP => OP::NOP,
            OP::NOP => OP::JMP,
            OP::ACC => panic!("found acc!"),
        };

        let (acc, last_index) = get_last_acc(&instructions);

        if last_index != instructions.len() {
            instructions[i].0 = op;
        } else {
            return acc as usize;
        }
    }

    // dbg!(&jmps);
    panic!("failed to terminate program");
}
