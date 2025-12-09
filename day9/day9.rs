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

fn part1(content: &str) {
    let parsed = parse(content);
    let mut max_area = 0;

    for i in 0..parsed.len() {
        let a = parsed[i];
        for j in i+1..parsed.len() {
            let b = parsed[j];
            let dx = 1 + (a.0 - b.0).abs();
            let dy = 1 + (a.1 - b.1).abs();
            let area = dx * dy;
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("Part 1: {}", max_area);
}

fn parse(content: &str) -> Vec<(i64, i64)> {
    content.split('\n').map(parse_coord).collect()
}

fn parse_coord(line: &str) -> (i64, i64) {
    let parts: Vec<i64> = line.split(',')
        .map(|part| part.parse().unwrap())
        .collect();
    (parts[0], parts[1])
}
