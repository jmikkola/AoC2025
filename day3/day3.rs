use std::env;
use std::fs;

fn main() {
    let content = get_content(get_arg());
    part1(&content);
}

fn part1(input: &str) {
    let mut total = 0;
    for line in input.split('\n') {
        let result = joltage(line);
        println!("{} {}", line, result);
        total += result;
    }

    println!("total: {}", total);
}

fn joltage(line: &str) -> u32 {
    let mut max = 0;
    let chars: Vec<char> = line.chars().collect();
    for i in 0..chars.len()-1 {
        let n_i = chars[i] as u32 - '0' as u32;
        for j in i+1..chars.len() {
            let n_j = chars[j] as u32 - '0' as u32;
            let joltage = 10 * n_i + n_j;
            if joltage > max {
                max = joltage;
            }
        }
    }
    max
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
