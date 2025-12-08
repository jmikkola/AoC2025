use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

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
    let coords = parse_coords(content);

    let mut distances = vec![];
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            let distance = find_distance_squared(coords[i], coords[j]);
            distances.push((i, j, distance));
        }
    }

    // Shortest distances first
    distances.sort_by_key(|d| d.2);

    let n_to_connect = if coords.len() < 100 { 10 } else { 1000 };

    if coords.len() < 100 {
        for d in distances.iter().take(10) {
            println!("{} -> {} ({:?} -> {:?}) is {}", d.0, d.1, coords[d.0], coords[d.1], d.2);
        }
    }

    let connections: Vec<(usize, usize)> = distances.iter()
        .take(n_to_connect)
        .map(|d| (d.0, d.1))
        .collect();

    let graph = as_graph(&connections);

    let mut group_sizes = find_groups(&graph, coords.len());

    group_sizes.sort_by_key(|n| -n);

    let mut result = 1;
    for i in 0..3 {
        result *= group_sizes[i];
    }
    println!("part 1: {}", result);
}

type Graph = HashMap<usize, Vec<usize>>;

fn find_groups(graph: &Graph, n_coords: usize) -> Vec<i64> {
    let mut group_sizes = vec![];
    let mut seen = HashSet::new();

    for start in 0..n_coords {
        if let Some(size) = traverse(graph, &mut seen, start) {
            group_sizes.push(size);
        }
    }

    group_sizes
}

fn traverse(graph: &Graph, seen: &mut HashSet<usize>, start: usize) -> Option<i64> {
    if seen.contains(&start) {
        return None;
    }

    let mut group = HashSet::new();
    let mut queue = vec![start];
    while let Some(next) = queue.pop() {
        group.insert(next);
        seen.insert(next);
        if let Some(neighbors) = graph.get(&next) {
            for neighbor in neighbors {
                if !seen.contains(neighbor) {
                    queue.push(*neighbor);
                }
            }
        }
    }

    Some(group.len() as i64)
}

fn as_graph(connections: &[(usize, usize)]) -> Graph {
    let mut graph: Graph = HashMap::new();
    for (i, j) in connections {
        if graph.contains_key(i) {
            graph.get_mut(i).unwrap().push(*j);
        } else {
            graph.insert(*i, vec![*j]);
        }

        if graph.contains_key(j) {
            graph.get_mut(j).unwrap().push(*i);
        } else {
            graph.insert(*j, vec![*i]);
        }
    }
    graph
}

fn find_distance_squared(a: Box, b: Box) -> i64 {
    let dx = a.0 - b.0;
    let dy = a.1 - b.1;
    let dz = a.2 - b.2;
    dx.pow(2) + dy.pow(2) + dz.pow(2)
}

type Box = (i64, i64, i64);

fn parse_coords(content: &str) -> Vec<Box> {
    content.split('\n').map(parse_coord).collect()
}

fn parse_coord(line: &str) -> Box {
    let parts: Vec<i64> = line.split(',')
        .map(|part| part.parse().unwrap())
        .collect();
    (parts[0], parts[1], parts[2])
}
