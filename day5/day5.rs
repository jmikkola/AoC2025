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


fn part2(input: &str) {
    let lines: Vec<&str> = input.split('\n').collect();

    let mut ranges: Vec<(u64, u64)> = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| to_range(line))
        .collect();
    ranges.sort();

    let mut disjoint_ranges: Vec<(u64, u64)> = vec![ranges[0]];

    for (lower, upper) in ranges.iter().skip(1) {
        let last = disjoint_ranges.last().unwrap();
        if *lower >= last.0 && *lower <= last.1 {
            // ranges overlap
            let last_idx = disjoint_ranges.len() - 1;
            disjoint_ranges[last_idx] = (last.0, last.1.max(*upper));
        } else {
            // ranges are disjoint
            disjoint_ranges.push((*lower, *upper));
        }
    }

    let mut n_fresh = 0;
    for (lower, upper) in disjoint_ranges.iter() {
        n_fresh += *upper - *lower + 1;
    }

    println!("Part 2: {}", n_fresh);
}
