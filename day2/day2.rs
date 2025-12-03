use std::env;
use std::fs;

fn main() {
    let content = read_input();
    part1(&content);
}

fn part1(input: &str) {
    let mut total = 0u64;
    for part in input.split(',') {
        let range = parse_part(part);
        let sum = find_invalid(range);
        total += sum;
    }
    println!("total: {}", total);
}

fn find_invalid(range: (u64, u64)) -> u64 {
    let (lower, upper) = range;

    let mut sum = 0;

    for n in lower..(upper+1) {
        if is_invalid(n) {
            sum += n;
        }
    }

    sum
}

fn is_invalid(n: u64) -> bool {
    let mut div = 10;
    while div < n {
        if (n / div) == (n % div) && (n % div) >= (div / 10) {
            return true;
        }
        div *= 10;
    }
    false
}

fn parse_part(part: &str) -> (u64, u64) {
    let numbers: Vec<u64> = part.split('-').map(|n| n.parse().unwrap()).collect();
    (numbers[0], numbers[1])
}

fn read_input() -> String {
    get_content(get_arg())
}

fn get_arg() -> String {
    match env::args().nth(1) {
        Some(f) => f,
        None => "example".into(),
    }
}

fn get_content(filename: String) -> String {
    fs::read_to_string(filename).unwrap().trim().to_string()
}
