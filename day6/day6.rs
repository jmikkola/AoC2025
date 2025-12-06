use std::env;
use std::fs;

fn main() {
    let content = get_content(get_arg());
    part1(&content);
    part2(&content);
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

fn part2(input: &str) {
    let lines: Vec<&str> = input.split('\n').collect();
    let chars: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut chars_flipped = vec![];
    for j in 0..chars[0].len() {
        let mut col = vec![];
        for i in 0..chars.len() {
            if j < chars[i].len() {
                col.push(chars[i][j]);
            } else {
                col.push(' ');
            }
        }
        let new_row: String = col.iter().collect();
        chars_flipped.push(new_row);
    }

    chars_flipped.push("  ".to_string());
    let mut acc = 0;
    let mut op = '+';
    let mut total = 0;

    for line in chars_flipped {
        if line.trim().is_empty() {
            total += acc;
            continue;
        }
        let n: u64 = line.trim().chars().take_while(|c| c.is_numeric()).collect::<String>().parse().unwrap();

        if line.trim().ends_with('+') {
            op = '+';
            acc = n;
        } else if line.trim().ends_with('*') {
            op = '*';
            acc = n;
        } else {
            if op == '+' {
                acc += n;
            } else {
                acc *= n;
            }
        }
    }

    println!("part 2: {}", total);
}
