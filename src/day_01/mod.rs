use crate::read_input;

pub fn solve() {
    let first = solve_first();
    dbg!(first);
}

fn solve_first() -> usize {
    read_input!()
        .into_iter()
        .map(usize_from_line)
        .sum()
}

fn usize_from_line(line: String) -> usize {
    let digits: Vec<usize> = line
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    10 * digits.first().unwrap() + digits.last().unwrap()
}
