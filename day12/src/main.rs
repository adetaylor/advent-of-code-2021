use std::collections::{HashMap, HashSet};

use itertools::Itertools;

static INPUT: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

static MEDIUM_INPUT: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

static BIG_INPUT: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

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
}

fn prefix_ok<'a>(path: &Vec<&'a str>) -> bool {
    let mut small_nodes_visited = HashSet::new();
    small_nodes_visited.insert("start");
    for (i, node) in path.iter().enumerate() {
        if i == 0 {
            continue;
        }
        if &node.to_lowercase() == *node {
            let newly_visited = small_nodes_visited.insert(node);
            if !newly_visited {
                return false;
            }
        }
    }
    true
}

static NO_PATHS: Vec<&str> = Vec::new();

fn find_paths(input: &str) -> usize {
    let edges = input.lines().map(Edge::parse);
    let edges = edges
        .map(|e| [e.flip(), e].into_iter())
        .flatten()
        .collect::<HashSet<_>>();
    let mut paths_from_node: HashMap<&str, Vec<_>> = HashMap::new();
    for e in edges {
        paths_from_node.entry(e.0).or_default().push(e.1);
    }
    let mut found_paths = Vec::new();
    let mut explored_prefixes = HashSet::new();
    let mut prefixes_to_explore = Vec::new();
    prefixes_to_explore.push(vec!["start"]);
    while let Some(prefix) = prefixes_to_explore.pop() {
        let new_nodes = paths_from_node
            .get(prefix.last().unwrap())
            .unwrap_or_else(|| &NO_PATHS);
        let new_prefixes = new_nodes
            .iter()
            .map(|new_node| {
                let mut new_prefix = prefix.clone();
                new_prefix.push(new_node);
                new_prefix
            })
            .filter(|prefix| !explored_prefixes.contains(prefix));
        for new_prefix in new_prefixes {
            if prefix_ok(&new_prefix) {
                if *new_prefix.last().unwrap() == "end" {
                    found_paths.push(new_prefix.clone());
                }
                prefixes_to_explore.push(new_prefix);
            }
        }
        explored_prefixes.insert(prefix);
    }
    println!("Paths={}", found_paths.len());
    found_paths.len()
}

fn main() {
    find_paths(INPUT);
}

#[test]
fn test_small() {
    assert_eq!(find_paths(INPUT), 10);
}

#[test]
fn test_medium() {
    assert_eq!(find_paths(MEDIUM_INPUT), 19);
}

#[test]
fn test_big() {
    assert_eq!(find_paths(BIG_INPUT), 226);
}