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
    fs::read_to_string(filename).unwrap().trim().to_string()
}

fn part1(text: &str) {
    let grid = to_grid(text);

    let mut n_rolls = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
                let min_i = if i > 0 { i - 1 } else { 0 };
                let min_j = if j > 0 { j - 1 } else { 0 };
                let max_i = if i < grid.len() - 1 { i + 1 } else { i };
                let max_j = if j < grid.len() - 1 { j + 1 } else { j };

                let mut n_neighbors = 0;
                for ii in min_i..max_i+1 {
                    for jj in min_j..max_j+1 {
                        if !(ii == i && jj == j) && grid[ii][jj] == '@' {
                            n_neighbors += 1;
                        }
                    }
                }

                if n_neighbors < 4 {
                    n_rolls += 1;
                    // print!("x");
                } else {
                    // print!("@");
                }
            } else {
                // print!(".");
            }
        }
        // println!("");
    }

    println!("Rolls: {}", n_rolls);
}


fn part2(text: &str) {
    let mut grid = to_grid(text);
    let mut n_rolls = 0;

    loop {
        let mut to_remove = vec![];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '@' {
                    let min_i = if i > 0 { i - 1 } else { 0 };
                    let min_j = if j > 0 { j - 1 } else { 0 };
                    let max_i = if i < grid.len() - 1 { i + 1 } else { i };
                    let max_j = if j < grid.len() - 1 { j + 1 } else { j };

                    let mut n_neighbors = 0;
                    for ii in min_i..max_i+1 {
                        for jj in min_j..max_j+1 {
                            if !(ii == i && jj == j) && grid[ii][jj] == '@' {
                                n_neighbors += 1;
                            }
                        }
                    }

                    if n_neighbors < 4 {
                        n_rolls += 1;
                        to_remove.push((i, j));
                    }
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        for (i, j) in to_remove.iter() {
            grid[*i][*j] = '.';
        }
    }

    println!("Rolls: {}", n_rolls);
}

fn to_grid(text: &str) -> Vec<Vec<char>> {
    text.split('\n')
        .map(|line| line.chars().collect())
        .collect()
}
