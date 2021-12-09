use std::collections::HashSet;

use itertools::Itertools;

static DEMO_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678
";

type Coords = (usize, usize);

fn char_to_int(c: char) -> u8 {
    c.to_digit(10).unwrap() as u8
}

fn adjacent_coords(pos: Coords, grid: &Vec<Vec<u8>>) -> impl Iterator<Item = (usize, usize)> {
    let (x, y) = pos;
    let w = grid.iter().next().unwrap().len();
    let h = grid.len();
    let mut nears = Vec::new();
    if x > 0 {
        nears.push((x - 1, y));
    }
    if y > 0 {
        nears.push((x, y - 1));
    }
    if x < w - 1 {
        nears.push((x + 1, y));
    }
    if y < h - 1 {
        nears.push((x, y + 1));
    }
    nears.into_iter()
}

fn adjacents(pos: Coords, grid: &Vec<Vec<u8>>) -> impl Iterator<Item = &u8> {
    adjacent_coords(pos, grid).map(|(x, y)| &grid[y][x])
}

fn main_part_one() {
    let grid = DEMO_INPUT
        .lines()
        .map(|l| l.chars().map(char_to_int).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let w = grid.iter().next().unwrap().len();
    let h = grid.len();
    let mut risk_sum = 0usize;
    for y in 0..h {
        for x in 0..w {
            let val = grid[y][x];
            let pos = (x, y);
            if adjacents(pos, &grid).all(|adj| *adj > val) {
                risk_sum += 1usize + val as usize;
            }
        }
    }
    println!("Val={}", risk_sum);
}

fn basin_size(pos: Coords, grid: &Vec<Vec<u8>>) -> usize {
    let mut explored = HashSet::new();
    let mut to_explore = Vec::new();
    explored.insert(pos);
    to_explore.push(pos);
    while let Some(pos) = to_explore.pop() {
        let mut additional = adjacent_coords(pos, grid)
            .filter(|(x, y)| {
                let val = grid[*y][*x];
                val < 9 && !explored.contains(&(*x, *y))
            })
            .collect::<Vec<_>>();
        explored.extend(additional.iter().cloned());
        to_explore.append(&mut additional);
    }
    explored.len()
}

fn main() {
    let grid = DEMO_INPUT
        .lines()
        .map(|l| l.chars().map(char_to_int).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let w = grid.iter().next().unwrap().len();
    let h = grid.len();
    let mut basin_sizes = (0..w)
        .cartesian_product(0..h)
        .filter(|pos| {
            let (x, y) = pos;
            let val = grid[*y][*x];
            adjacents(*pos, &grid).all(|adj| *adj > val)
        })
        .map(|pos| basin_size(pos, &grid))
        .collect::<Vec<_>>();
    basin_sizes.sort();
    basin_sizes.reverse();
    println!("Basins {:?}", basin_sizes);
    println!(
        "Size product={}",
        basin_sizes[0..3].iter().product::<usize>()
    );
}
