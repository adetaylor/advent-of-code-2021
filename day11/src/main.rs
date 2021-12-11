use itertools::Itertools;

static INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

fn adjacents(x: usize, y: usize, grid: &Vec<Vec<u32>>) -> impl Iterator<Item = (usize, usize)> {
    let w = grid[0].len();
    let h = grid.len();
    (-1isize..=1isize)
        .cartesian_product(-1isize..=1isize)
        .filter(|(dx, dy)| *dx != 0 || *dy != 0)
        .map(move |(dx, dy)| (x as isize + dx, y as isize + dy))
        .filter(move |(x, y)| {
            *x >= 0isize && *y >= 0isize && *x < (w as isize) && *y < (h as isize)
        })
        .map(|(x, y)| (x as usize, y as usize))
}

fn main() {
    let mut grid = INPUT
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let w = grid[0].len();
    let h = grid.len();
    let mut total_flashes = 0usize;
    for step in 1..=10000 {
        let mut flashes_this_try = 0usize;
        for cell in grid.iter_mut().map(|row| row.iter_mut()).flatten() {
            *cell += 1;
        }
        let mut flashers = get_flashers(w, h, &grid);
        while !flashers.is_empty() {
            for (x, y) in flashers.into_iter() {
                flashes_this_try += 1;
                grid[y][x] = u32::MAX;
                for (adj_x, adj_y) in adjacents(x, y, &grid) {
                    if grid[adj_y][adj_x] != u32::MAX {
                        grid[adj_y][adj_x] += 1;
                    }
                }
            }
            flashers = get_flashers(w, h, &grid);
        }
        for cell in grid.iter_mut().map(|row| row.iter_mut()).flatten() {
            if *cell > 9 {
                *cell = 0;
            }
        }
        println!("Flashes on step {} = {}", step, flashes_this_try);
        total_flashes += flashes_this_try;
        if flashes_this_try == w * h {
            println!("First total flash={}", step);
            break;
        }
    }
    println!("Flashes to step 100 = {}", total_flashes);
}

fn get_flashers(w: usize, h: usize, grid: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    (0..w)
        .cartesian_product(0..h)
        .filter(|(x, y)| grid[*y][*x] > 9 && grid[*y][*x] != u32::MAX)
        .collect_vec()
}
