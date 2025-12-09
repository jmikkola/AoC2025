use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::hash::Hash;

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
    let coords = parse_coords(content);

    let distances = find_distances(&coords);

    let n_to_connect = if coords.len() < 100 { 10 } else { 1000 };

    // if coords.len() < 100 {
    //     for d in distances.iter().take(10) {
    //         println!("{} -> {} ({:?} -> {:?}) is {}", d.0, d.1, coords[d.0], coords[d.1], d.2);
    //     }
    // }

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


fn part2(content: &str) {
    let coords = parse_coords(content);
    let distances = find_distances(&coords);
    let mut disjoint_set = DisjointSet::new();
    let edges = disjoint_set.kruskal(&distances);

    // for edge in edges.iter() {
    //     println!("({},{}): {:?} -> {:?}", edge.0, edge.1, coords[edge.0], coords[edge.1]);
    // }

    let last_edge = edges[edges.len() - 1];
    let last_x_pair = coords[last_edge.0].0 * coords[last_edge.1].0;
    println!("part 2: {}", last_x_pair);
}


fn find_distances(coords: &[Box]) -> Vec<(usize, usize, i64)> {
    let mut distances = vec![];
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            let distance = find_distance_squared(coords[i], coords[j]);
            distances.push((i, j, distance));
        }
    }

    // Shortest distances first
    distances.sort_by_key(|d| d.2);
    distances
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

struct DisjointSet<T> {
    nodes: HashMap<T, DSNode<T>>,
}

struct DSNode<T> {
    parent: T,
    size: usize,
}

impl<T: Clone+Ord+Hash+std::fmt::Debug> DisjointSet<T> {
    fn new() -> Self {
        DisjointSet{
            nodes: HashMap::new(),
        }
    }

    fn kruskal(&mut self, distances: &[(T, T, i64)]) -> Vec<(T, T)> {
        for (i, j, _) in distances.iter() {
            self.make_set(i);
            self.make_set(j);
        }

        let mut edges = vec![];
        for (u, v, _) in distances.iter() {
            let u_root = self.find(u);
            let v_root = self.find(v);
            if u_root != v_root {
                edges.push((u.clone(), v.clone()));
                self.union(&u_root, &v_root);
            }
        }
        edges
    }

    fn make_set(&mut self, x: &T) {
        if !self.nodes.contains_key(x) {
            self.nodes.insert(x.clone(), DSNode{
                parent: x.clone(),
                size: 1,
            });
        }
    }

    fn find(&mut self, x: &T) -> T {
        let parent = {
            let node = self.nodes.get(x).unwrap();
            node.parent.clone()
        };
        if parent != *x {
            let new_parent = self.find(&parent);
            let node_mut = self.nodes.get_mut(x).unwrap();
            node_mut.parent = new_parent.clone();
            return new_parent;
        } else {
            return x.clone();
        }
    }

    fn union(&mut self, x: &T, y: &T) {
        let x_root = self.find(x);
        let y_root = self.find(y);

        if x_root == y_root {
            return;
        }

        let x_size: usize = self.nodes.get(&x_root).unwrap().size;
        let y_size: usize = self.nodes.get(&y_root).unwrap().size;

        let (new_root, other) = if x_size >= y_size {
            (x_root, y_root)
        } else {
            (y_root, x_root)
        };

        self.nodes.get_mut(&other).unwrap().parent = new_root.clone();
        self.nodes.get_mut(&new_root).unwrap().size = x_size + y_size;
    }
}
