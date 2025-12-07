use std::env;
use std::fs;
use std::collections::HashSet;

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
    let chars: Vec<Vec<char>> = input.split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let mut splits = 0;
    let start_beam = chars[0].iter().position(|&c| c == 'S').unwrap();
    let mut beams = HashSet::new();
    beams.insert(start_beam);

    println!("{}", chars[0].iter().collect::<String>());
    for row in chars.iter().skip(1) {
        let mut temp_row = row.clone();
        let mut next_beams = HashSet::new();
        for &beam in beams.iter() {
            if row[beam] == '.' {
                temp_row[beam] = '|';
                next_beams.insert(beam);
            } else if row[beam] == '^' {
                splits += 1;
                let lower = beam.checked_sub(1);
                if let Some(lower_beam) = lower {
                    temp_row[lower_beam] = '|';
                    next_beams.insert(lower_beam);
                }
                let upper = beam + 1;
                if upper < row.len() {
                    temp_row[upper] = '|';
                    next_beams.insert(upper);
                }
            }
        }

        println!("{}", temp_row.iter().collect::<String>());
        beams = next_beams;
    }

    println!("part 1: {}", splits);
}
