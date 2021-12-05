static DEMO_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

#[derive(Debug)]
struct Point(pub isize, pub isize);

impl Point {
    fn from(input: &str) -> Self {
        let (x, y) = input.split_once(',').unwrap();
        Self(x.parse().unwrap(), y.parse().unwrap())
    }
    fn x(&self) -> isize {
        self.0
    }
    fn y(&self) -> isize {
        self.1
    }
}

#[derive(Debug)]
struct Line(pub Point, pub Point);

impl Line {
    fn from(a: Point, b: Point) -> Self {
        if a.x() < b.x() {
            Self(a, b)
        } else if a.x() == b.x() {
            if a.y() < b.y() {
                Self(a, b)
            } else {
                Self(b, a)
            }
        } else {
            Self(b, a)
        }
    }
    fn points(&self) -> impl Iterator<Item = &Point> {
        [&self.0, &self.1].to_vec().into_iter()
    }
}

fn main_part_one() {
    let lines = get_lines();
    let mut grid = make_grid(&lines);
    for l in lines {
        if l.0.y() == l.1.y() {
            // Horizontal
            for x in l.0.x()..l.1.x() + 1 {
                grid[l.0.y() as usize][x as usize] += 1usize;
            }
        } else if l.0.x() == l.1.x() {
            // Vertical
            for y in l.0.y()..l.1.y() + 1 {
                grid[y as usize][l.0.x() as usize] += 1usize;
            }
        }
    }
    println!("Grid is {:?}", grid);
    let total_overlap = grid
        .into_iter()
        .map(|row| row.into_iter().filter(|count| *count > 1usize))
        .flatten()
        .count();
    println!("Total overlap = {}", total_overlap);
}

fn get_lines() -> Vec<Line> {
    DEMO_INPUT
        .lines()
        .map(|l| {
            let (top_left, bottom_right) = l.split_once(" -> ").unwrap();
            Line::from(Point::from(top_left), Point::from(bottom_right))
        })
        .collect::<Vec<_>>()
}

fn make_grid(lines: &[Line]) -> Vec<Vec<usize>> {
    let max_x = lines
        .iter()
        .map(Line::points)
        .flatten()
        .map(|p| p.0)
        .max()
        .unwrap();
    let max_y = lines
        .iter()
        .map(Line::points)
        .flatten()
        .map(|p| p.1)
        .max()
        .unwrap();
    std::iter::repeat(
        std::iter::repeat(0usize)
            .take(max_x as usize + 1)
            .collect::<Vec<_>>(),
    )
    .take(max_y as usize + 1)
    .collect::<Vec<_>>()
}

fn print_grid(grid: &Vec<Vec<usize>>) {
    for row in grid {
        println!("{:?}", row);
    }
}

fn main() {
    let lines = get_lines();
    let mut grid = make_grid(&lines);
    for l in lines {
        let dx = l.1.x() - l.0.x();
        let dy = l.1.y() - l.0.y();
        if dy == 0 {
            // Horizontal
            for x in l.0.x()..l.1.x() + 1 {
                grid[l.0.y() as usize][x as usize] += 1usize;
            }
        } else if dx == 0 {
            // Vertical
            for y in l.0.y()..l.1.y() + 1 {
                grid[y as usize][l.0.x() as usize] += 1usize;
            }
        } else if dx == dy {
            // Top left to bottom right
            for (i, x) in (l.0.x()..l.1.x() + 1).enumerate() {
                grid[l.0.y() as usize + i][x as usize] += 1usize;
            }
        } else if dx == -dy {
            for (i, x) in (l.0.x()..l.1.x() + 1).enumerate() {
                grid[l.0.y() as usize - i][x as usize] += 1usize;
            }
        }
    }
    print_grid(&grid);
    let total_overlap = grid
        .into_iter()
        .map(|row| row.into_iter().filter(|count| *count > 1usize))
        .flatten()
        .count();
    println!("Total overlap = {}", total_overlap);
}
