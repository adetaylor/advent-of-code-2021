static INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

fn closes(opening: char) -> char {
    match opening {
        '[' => ']',
        '{' => '}',
        '<' => '>',
        '(' => ')',
        _ => panic!("unexpected"),
    }
}

fn is_pair(opening: char, closing: char) -> bool {
    closing == closes(opening)
}

fn first_error(line: &str) -> Option<char> {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '[' | '{' | '<' | '(' => stack.push(c),
            _ => match stack.pop() {
                None => return None,
                Some(opening) if !is_pair(opening, c) => return Some(c),
                _ => {}
            },
        }
    }
    None
}

fn line_error(line: &str) -> u32 {
    let score = match first_error(line) {
        None => 0,
        Some(')') => 3,
        Some(']') => 57,
        Some('}') => 1197,
        Some('>') => 25137,
        Some(c) => panic!("unexpected {}", c),
    };
    println!("Line {} score {}", line, score);
    score
}

fn main_part_one() {
    let score: u32 = INPUT.lines().map(line_error).sum();
    println!("Score = {}", score);
}

fn complete_line(line: &str) -> Option<u64> {
    let mut stack = Vec::new();
    for c in line.chars() {
        match c {
            '[' | '{' | '<' | '(' => stack.push(c),
            _ => match stack.pop() {
                Some(opening) if is_pair(opening, c) => {}
                None => return None,    // corrupt - more closings than openings
                Some(_) => return None, // corrupt - mismatches
                _ => {}
            },
        }
    }
    // incomplete - more openings than closings
    Some(
        stack
            .into_iter()
            .rev()
            .fold(0u64, |mut accumulator, opening| {
                accumulator *= 5u64;
                accumulator += match opening {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("unexpected"),
                };
                accumulator
            }),
    )
}

fn main() {
    let mut scores = INPUT
        .lines()
        .map(complete_line)
        .flatten()
        .collect::<Vec<_>>();
    scores.sort();
    println!("Score = {}", scores[scores.len() / 2]);
}
