use std::collections::HashSet;

use itertools::Itertools;

static TINY_INPUT: &str = "start-A
b-A
start-b
A-end";

static INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

#[derive(Hash, Eq, PartialEq)]
struct Edge<'a>(&'a str, &'a str);

impl<'a> Edge<'a> {
    fn parse(input: &'a str) -> Self {
        let (a, b) = input.split_once('-').unwrap();
        Self(a, b)
    }
    fn flip(&self) -> Self {
        Edge(self.1, self.0)
    }
    fn nodes(&self) -> impl Iterator<Item = &str> {
        [self.0, self.1].into_iter()
    }
}

fn path_ok<'a>(path: &Vec<&'a &'a str>, edges: &HashSet<Edge>) -> bool {
    if *path[0] != "start" {
        return false;
    }
    if **path.last().unwrap() != "end" {
        return false;
    }
    let mut small_nodes_visited = HashSet::new();
    small_nodes_visited.insert("start");
    for (i, node) in path.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let e = Edge(path[i - 1], node);
        if !edges.contains(&e) {
            return false;
        }
        if &node.to_lowercase() == *node {
            let newly_visited = small_nodes_visited.insert(node);
            if !newly_visited {
                return false;
            }
        }
    }
    println!("OK path {:?}", path);
    true
}

fn main() {
    let edges = TINY_INPUT.lines().map(Edge::parse);
    let edges = edges
        .map(|e| [e.flip(), e].into_iter())
        .flatten()
        .collect::<HashSet<_>>();
    let nodes = edges.iter().map(Edge::nodes).flatten().collect_vec();
    let possible_path_lengths = 1..nodes.len();
    let paths = possible_path_lengths
        .into_iter()
        .map(|l| nodes.iter().permutations(l))
        .flatten();
    // println!("Paths to consider {}", paths.count());
    let ok_paths = paths
        .filter(|p| path_ok(p, &edges))
        .collect::<HashSet<_>>();
    println!("Paths={}", ok_paths.len());
}
