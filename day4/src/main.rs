static DEMO_INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

use itertools::Itertools;

struct Number {
    pub val: i32,
    pub marked: bool,
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.marked {
            write!(f, "*{}*", self.val)
        } else {
            write!(f, "{}", self.val)
        }
    }
}

impl Number {
    fn new(val: i32) -> Self {
        Self { val, marked: false }
    }
    fn mark(&mut self, num: i32) {
        if self.val == num {
            self.marked = true;
        }
    }
    fn get_unmarked_total(&self) -> i32 {
        if self.marked {
            0
        } else {
            self.val
        }
    }
}

#[derive(Debug)]
struct Row(Vec<Number>);

impl Row {
    fn new(numbers: Vec<Number>) -> Self {
        Self(numbers)
    }
    fn mark(&mut self, num: i32) {
        for nums in &mut self.0 {
            nums.mark(num);
        }
    }
    fn get_unmarked_total(&self) -> i32 {
        self.0
            .iter()
            .fold(0, |total, number| total + number.get_unmarked_total())
    }
    fn is_complete(&self) -> bool {
        self.0.iter().all(|n| n.marked)
    }
}

struct Board(Vec<Row>);

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in &self.0 {
            writeln!(f, "{:?}", r)?;
        }
        Ok(())
    }
}

impl Board {
    fn new(rows: Vec<Row>) -> Self {
        Self(rows)
    }
    fn mark(&mut self, num: i32) {
        for rows in &mut self.0 {
            rows.mark(num);
        }
    }
    fn is_complete(&self) -> bool {
        if self.0.iter().any(Row::is_complete) {
            return true;
        }
        let num_cols = self.0[0].0.len();
        for col in 0..num_cols {
            if self.0.iter().all(|row| row.0[col].marked) {
                return true;
            }
        }
        false
    }
    fn get_unmarked_total(&self) -> i32 {
        self.0
            .iter()
            .fold(0, |total, row| total + row.get_unmarked_total())
    }
}

fn parse_input() -> (Vec<Board>, impl Iterator<Item = i32>) {
    let mut lines = DEMO_INPUT.split_terminator('\n');
    let random_numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap());
    let boards = lines.chunks(6);
    let boards = boards.into_iter();
    let boards = boards
        .map(|mut board_lines| {
            board_lines.next();
            Board::new(
                board_lines
                    .map(|line| {
                        Row::new(
                            line.split_whitespace()
                                .map(|s| Number::new(s.parse::<i32>().unwrap()))
                                .collect(),
                        )
                    })
                    .collect(),
            )
        })
        .collect::<Vec<_>>();
    (boards, random_numbers)
}

fn main_part_one() {
    let (mut boards, random_numbers) = parse_input();
    let mut won = false;
    for random_number in random_numbers {
        for board in &mut boards {
            board.mark(random_number);
        }
        for board in &boards {
            if board.is_complete() {
                let unmarked_sum = board.get_unmarked_total();
                println!(
                    "Sum of unmarked {}, called number {}, product {}",
                    unmarked_sum,
                    random_number,
                    unmarked_sum * random_number
                );
                won = true;
                break;
            }
        }
        if won {
            break;
        }
    }
}

fn main() {
    let (mut boards, random_numbers) = parse_input();
    for random_number in random_numbers {
        for board in &mut boards {
            board.mark(random_number);
        }
        if boards.len() == 1 && boards[0].is_complete() {
            let losing_board = &boards[0];
            let unmarked_sum = losing_board.get_unmarked_total();
            println!(
                "Sum of unmarked {}, called number {}, product {}",
                unmarked_sum,
                random_number,
                unmarked_sum * random_number
            );
            break;
        }
        boards.retain(|b| !b.is_complete());
    }
}
