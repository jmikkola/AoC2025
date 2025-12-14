use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

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

fn part2(content: &str) {
    let parsed = parse(content);
    let mut max_area = 0;

    let crushed = crush_coords(&parsed);
    let fill = fill_area(&crushed);

    for i in 0..parsed.len() {
        let a = parsed[i];
        for j in i+1..parsed.len() {
            let b = parsed[j];
            let dx = 1 + (a.0 - b.0).abs();
            let dy = 1 + (a.1 - b.1).abs();
            let area = dx * dy;

            if inside(&crushed, &fill, i, j) && area > max_area {
                max_area = area;
            }
        }
    }

    println!("Part 2: {}", max_area);
}

// Create an area-reduced representation of the grid by crushing the values
// together as mush as possible without losing relative ordering.
fn crush_coords(parsed: &[(i64, i64)]) -> Vec<(i64, i64)> {
    let (xs, ys): (Vec<i64>, Vec<i64>) = parsed.iter().cloned().unzip();
    let x_crushed = crush(&xs);
    let y_crushed = crush(&ys);
    return x_crushed.iter().cloned().zip(y_crushed).collect();
}

// "crush" the values together by replacing the nth greatest value with n
fn crush(values: &[i64]) -> Vec<i64> {
    let mut in_order: Vec<i64> = values.to_vec();
    in_order.sort();

    let mut to_crushed = HashMap::new();
    let mut last_value = -1;

    for value in in_order {
        if value != last_value {
            to_crushed.insert(value, to_crushed.len() as i64 + 1);
            last_value = value;
        }
    }

    values.iter()
        .map(|val| to_crushed.get(val).unwrap().clone())
        .collect()
}

fn fill_area(crushed: &[(i64, i64)]) -> Vec<Vec<bool>> {
    // println!("{:?}", crushed);

    let max_x = crushed.iter().map(|a| a.0).max().unwrap() as usize;
    let max_y = crushed.iter().map(|a| a.1).max().unwrap() as usize;

    let mut lines = vec![];
    for _ in 0..=max_y {
        lines.push(vec![false; max_x + 1]);
    }

    // Draw the lines between the red tiles
    let mut prev_idx = crushed.len() - 1;
    for i in 0..crushed.len() {
        let start = crushed[prev_idx];
        let end = crushed[i];

        let xmin = start.0.min(end.0) as usize;
        let xmax = start.0.max(end.0) as usize;
        let ymin = start.1.min(end.1) as usize;
        let ymax = start.1.max(end.1) as usize;

        if xmin == xmax {
            for y in ymin..=ymax {
                lines[y][xmin] = true;
            }
        } else if ymin == ymax {
            for x in xmin..=xmax {
                lines[ymin][x] = true;
            }
        } else {
            panic!("points {} and {} were not in a orthogonal line", prev_idx, i);
        }

        prev_idx = i;
    }

    print_area(&lines);

    // Add an extra row and col at the end to allow the flood-fill to wrap around the shape
    let mut area = vec![];
    for _ in 0..=max_y+1 {
        area.push(vec![true; max_x + 2]);
    }

    if lines[0][0] {
        panic!("this relies on the upper left corner being empty");
    }

    let mut queue: Vec<(i64, i64)> = vec![(0, 0)];
    let mut seen = HashSet::new();
    while let Some((x, y)) = queue.pop() {
        // Don't cross into the inside of the shape
        if (y as usize) < lines.len() && (x as usize) < lines[0].len() {
            if lines[y as usize][x as usize] {
                continue;
            }
        }

        if seen.contains(&(x, y)) {
            continue;
        }
        seen.insert((x, y));

        // Flood-fill falses everywhere reachable
        area[y as usize][x as usize] = false;

        // Find neighbors
        for ny in (y - 1).max(0)..=(y + 1).min(area.len() as i64 - 1) {
            for nx in (x - 1).max(0)..=(x + 1).min(area[0].len() as i64 - 1) {
                if !(ny == y && nx == x) {
                    queue.push((nx, ny));
                }
            }
        }
    }

    println!();
    print_area(&area);

    area
}

fn print_area(area: &[Vec<bool>]) {
    if area.len() > 10 {
        return;
    }
    for row in area.iter() {
        for &col in row.iter() {
            if col {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn inside(crushed: &[(i64, i64)], fill: &[Vec<bool>], i: usize, j: usize) -> bool {
    let a = crushed[i];
    let b = crushed[j];
    let xmin = a.0.min(b.0);
    let xmax = a.0.max(b.0);
    let ymin = a.1.min(b.1);
    let ymax = a.1.max(b.1);

    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if !fill[y as usize][x as usize] {
                return false;
            }
        }
    }


    true
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
