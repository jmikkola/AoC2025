use std::env;
use std::fs;

fn main() {
    let content = get_content(get_arg());
    part1(&content);
}

fn get_arg() -> String {
    match env::args().nth(1) {
        Some(f) => f,
        None => "example".into(),
    }
}

fn get_content(filename: String) -> String {
    fs::read_to_string(filename).expect("error reading file").trim().to_string()
}

fn part1(input: &str) {
    let lines: Vec<&str> = input.split('\n').collect();

    let ranges: Vec<(u64, u64)> = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| to_range(line))
        .collect();

    let mut n_fresh = 0;

    lines.iter()
        .skip_while(|line| line.contains('-') || line.is_empty())
        .for_each(|line| {
            let id: u64 = line.parse().unwrap();
            if in_any_range(id, &ranges) {
                n_fresh += 1;
            }
        });

    println!("Part 1: {}", n_fresh);
}

fn in_any_range(id: u64, ranges: &[(u64, u64)]) -> bool {
    for (lower, upper) in ranges.iter() {
        if id >= *lower && id <= *upper {
            return true;
        }
    }
    false
}

fn to_range(line: &str) -> (u64, u64) {
    let parts: Vec<u64> = line.split('-').map(|part| part.parse().unwrap()).collect();
    (parts[0], parts[1])
}
