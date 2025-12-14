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

    let mut total = 0;
    for line in parsed.iter() {
        // println!("{:?}", line);
        total += presses(line);
    }

    println!("part 1: {}", total);
}

fn parse(content: &str) -> Vec<Line> {
    content.split('\n')
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Line {
    // E.g. "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
    let (lights_part, buttons_and_values) = line.split_once("]").unwrap();

    let lights: Vec<bool> = lights_part.trim_start_matches("[")
        .chars()
        .map(|c| c == '#')
        .collect();

    let (buttons_part, _values_part) = buttons_and_values.split_once("{").unwrap();
    let buttons = buttons_part.trim()
        .split(" ")
        .map(parse_button)
        .collect();

    Line{
        lights: lights,
        buttons: buttons,
    }
}

fn parse_button(button: &str) -> Vec<usize> {
    button
        .trim_start_matches("(")
        .trim_end_matches(")")
        .split(",")
        .map(|idx| idx.parse().unwrap())
        .collect()
}

fn presses(line: &Line) -> usize {
    for n_buttons in 1..line.buttons.len() {
        let current = vec![false; line.lights.len()];
        if try_n_buttons(line, n_buttons, current) {
            return n_buttons;
        }
    }

    panic!("could not find solution for {:?}", line);
}

fn try_n_buttons(line: &Line, n_buttons: usize, so_far: Vec<bool>) -> bool {
    if n_buttons == 0 {
        return line.lights == so_far;
    }

    for i in 0..line.buttons.len() {
        let mut button_result = so_far.clone();
        for &light in line.buttons[i].iter() {
            button_result[light] = !button_result[light];
        }
        if try_n_buttons(line, n_buttons - 1, button_result) {
            return true;
        }
    }

    false
}

#[derive(Debug)]
struct Line {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>
}
