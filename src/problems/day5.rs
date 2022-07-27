use itertools::Itertools;

fn seat_ids(file: &str) -> impl Iterator<Item = usize> + '_ {
    file.lines().map(|l| {
        let row = l[..7].replace('F', "0").replace('B', "1");
        let col = l[7..].replace('L', "0").replace('R', "1");

        let row = usize::from_str_radix(&row, 2).unwrap();
        let col = usize::from_str_radix(&col, 2).unwrap();

        row * 8 + col
    })
}

pub fn problem_a() -> usize {
    let file = std::fs::read_to_string("res/day5.txt").unwrap();
    seat_ids(&file).max().unwrap()
}

pub fn problem_b() -> usize {
    let file = std::fs::read_to_string("res/day5.txt").unwrap();
    seat_ids(&file)
        .sorted()
        .tuple_windows()
        .find(|&(id1, id2)| id2 == id1 + 2)
        .map(|(id1, _)| id1 + 1)
        .unwrap()
}
