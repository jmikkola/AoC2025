use std::env;
use std::fs;

fn main() {
    let content = get_content(get_arg());
    part1(&content);
    // part2(&content);
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
    let parts: Vec<Vec<&str>> = lines.iter().map(|line| line.split_whitespace().collect()).collect();
    // println!("{:?}", parts);

    let n_rows = parts.len();

    let mut total = 0;
    for i in 0..parts[0].len() {
        let mut acc: u64 = 0;
        let op = parts[n_rows - 1][i];
        if parts.last().unwrap()[i] == "*" {
            acc = 1;
            for j in 0..n_rows-1 {
                acc *= parts[j][i].parse::<u64>().unwrap();
            }
        } else if op == "+" {
            for j in 0..n_rows-1 {
                acc += parts[j][i].parse::<u64>().unwrap();
            }
        } else {
            panic!("what is {}", op);
        }

        total += acc;
    }

    println!("part 1: {}", total);
}

// fn part2(input: &str) {
// }
