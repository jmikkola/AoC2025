use std::env;
use std::fs;

fn main() {
    let content = get_content(get_arg());
    part1(&content);
    part2(&content);
}

fn part1(input: &str) {
    let mut total = 0;
    for line in input.split('\n') {
        let result = joltage(line);
        total += result;
    }

    println!("part 1: {}", total);
}

fn part2(input: &str) {
    let mut total = 0;
    for line in input.split('\n') {
        let result = joltage2(line);
        total += result;
    }

    println!("part 2: {}", total);
}

fn joltage2(line: &str) -> u64 {
    let chars: Vec<char> = line.chars().collect();
    find_max_joltage(12, 0, &chars)
}

fn find_max_joltage(n: usize, so_far: u64, chars: &[char]) -> u64 {
    if n == 0 {
        return so_far;
    }

    let mut best = 0;
    let mut rest_chars = &chars[1..];

    for i in 0..chars.len()-n+1 {
        let char_value = chars[i] as u64 - '0' as u64;
        if char_value > best {
            best = char_value;
            rest_chars = &chars[i+1..];
        }
    }

    find_max_joltage(n - 1, so_far * 10 + best, rest_chars)
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
