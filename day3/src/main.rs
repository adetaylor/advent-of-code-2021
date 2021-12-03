static DEMO_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

fn main_part_one() {
    let mut input = DEMO_INPUT.split_whitespace().peekable();
    let num_bits = input.peek().unwrap().len();
    let mut total_lines = 0;
    let mut num_ones = std::iter::repeat(0).take(num_bits).collect::<Vec<_>>();
    for i in input {
        total_lines += 1;
        for (i, val) in i.chars().enumerate() {
            if val == '1' {
                num_ones[i] += 1;
            }
        }
    }
    let mut gamma = 0;
    let mut epsilon = 0;
    let threshold = total_lines / 2;
    for (pos, ones_count) in num_ones.into_iter().enumerate() {
        if ones_count > threshold {
            gamma += 2i64.pow(num_bits as u32 - pos as u32 - 1);
        } else {
            epsilon += 2i64.pow(num_bits as u32 - pos as u32 - 1);
        }
    }
    println!(
        "Gamma: {}, epsilon: {}, product: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn main() {
    let input_vec = DEMO_INPUT.split_whitespace().collect::<Vec<_>>();
    let num_digits = input_vec[0].len();
    let oxygen_rating = filter_list(input_vec.clone(), false, num_digits);
    println!("Oxy: {}", oxygen_rating);
    let co2_rating = filter_list(input_vec, true, num_digits);
    println!("co2: {}", co2_rating);
    let product = oxygen_rating * co2_rating;
    println!("product: {}", product);
}

fn get_most_common_by_digit(input: &[&str], digit_num: usize) -> char {
    let total_lines = input.len();
    let num_ones = input
        .iter()
        .filter(|item| item.chars().nth(digit_num).unwrap() == '1')
        .count();
    let num_zeroes = total_lines - num_ones;
    if num_ones < num_zeroes {
        '0'
    } else {
        '1'
    }
}

fn filter_list(mut possibilities: Vec<&str>, negate: bool, num_digits: usize) -> u64 {
    for i in 0..num_digits {
        let most_common = get_most_common_by_digit(&possibilities, i);
        possibilities.retain(|num| {
            let relevant_char = num.chars().nth(i).unwrap();
            (relevant_char == most_common) ^ negate
        });
        if possibilities.len() == 1 {
            break;
        }
    }
    u64::from_str_radix(possibilities[0], 2).unwrap()
}
