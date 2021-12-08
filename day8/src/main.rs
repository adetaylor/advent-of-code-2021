use std::collections::{HashMap, HashSet};

static SIMPLE_INPUT: &str =
    "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |cdfeb fcadb cdfeb cdbaf";
static BIG_INPUT: &str =
    "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |fgae cfgab fg bagce";

type SegmentSet = HashSet<char>;

struct Entry {
    pub signal_patterns: Vec<SegmentSet>,
    pub output_value: Vec<SegmentSet>,
}

impl Entry {
    fn new(line: &str) -> Self {
        let (signal_patterns, output_value) = line.split_once('|').unwrap();
        let signal_patterns = signal_patterns
            .split_whitespace()
            .map(|s| s.chars().collect())
            .collect();
        let output_value = output_value
            .split_whitespace()
            .map(|s| s.chars().collect())
            .collect();
        Self {
            signal_patterns,
            output_value,
        }
    }

    fn deduce_digits(&self) -> HashMap<u8, SegmentSet> {
        // The answer we want to assemble - the pattern of segments for each digit.
        let mut digit_patterns: HashMap<u8, SegmentSet> = HashMap::new();
        // Some we know based on the number of illuminated segments.
        let signal_patterns_by_length: HashMap<usize, &SegmentSet> =
            self.signal_patterns.iter().map(|p| (p.len(), p)).collect();
        Self::insert_known_segment_count_pattern(
            &mut digit_patterns,
            7,
            3,
            &signal_patterns_by_length,
        );
        Self::insert_known_segment_count_pattern(
            &mut digit_patterns,
            1,
            2,
            &signal_patterns_by_length,
        );
        Self::insert_known_segment_count_pattern(
            &mut digit_patterns,
            4,
            4,
            &signal_patterns_by_length,
        );
        Self::insert_known_segment_count_pattern(
            &mut digit_patterns,
            8,
            7,
            &signal_patterns_by_length,
        );
        // Now let's work out which segment is which.
        // We know the top segment, because it's present in 7 but not 1.
        let top = (digit_patterns.get(&7u8).unwrap() - digit_patterns.get(&1u8).unwrap())
            .into_iter()
            .next()
            .unwrap();
        // We now do frequency analysis on the remaining segments.
        let mut count_by_char: HashMap<char, usize> = HashMap::new();
        for ch in self.signal_patterns.iter().flatten() {
            *count_by_char.entry(*ch).or_default() += 1;
        }
        let mut chars_by_count: HashMap<usize, Vec<char>> = HashMap::new();
        for (ch, count) in count_by_char.iter() {
            chars_by_count.entry(*count).or_default().push(*ch);
        }
        // Three segments have a unique number of occurrences across
        // the various digits.
        let br = Self::get_unique_segment_frequency(&chars_by_count, 9);
        let bl = Self::get_unique_segment_frequency(&chars_by_count, 4);
        let tl = Self::get_unique_segment_frequency(&chars_by_count, 6);
        // Top right is illuminated as frequently as top, but we already know
        // which segment is top.
        let mut tr_possibles = Self::get_segment_frequency(&chars_by_count, 8);
        tr_possibles.remove(&top);
        // So there's only one possibility left.
        let tr = tr_possibles.into_iter().next().unwrap();
        // Next we work out the middle segment.
        // We know it appears within the digit 4, and we know it isn't
        // top left, top right or bottom right.
        let mut mid_possibles = digit_patterns.get(&4u8).unwrap().clone();
        mid_possibles.remove(&tl);
        mid_possibles.remove(&tr);
        mid_possibles.remove(&br);
        // So again there's only one possibility left.
        let mid = mid_possibles.into_iter().next().unwrap();
        // Finally the bottom segment. It appears in 7 digits;
        // so does the middle segment but we can eliminate that.
        let mut bot_possibles = Self::get_segment_frequency(&chars_by_count, 7);
        bot_possibles.remove(&mid);
        let bot = bot_possibles.into_iter().next().unwrap();
        // So now we know which segment is which, we can fill in the
        // pattern for each remaining digit.
        Self::insert_digit_pattern(&mut digit_patterns, 0u8, &[top, tl, tr, bl, br, bot]);
        Self::insert_digit_pattern(&mut digit_patterns, 2u8, &[top, tr, mid, bl, bot]);
        Self::insert_digit_pattern(&mut digit_patterns, 3u8, &[top, tr, mid, br, bot]);
        Self::insert_digit_pattern(&mut digit_patterns, 5u8, &[top, tl, mid, br, bot]);
        Self::insert_digit_pattern(&mut digit_patterns, 6u8, &[top, tl, mid, bl, br, bot]);
        Self::insert_digit_pattern(&mut digit_patterns, 9u8, &[top, tl, tr, mid, br, bot]);
        digit_patterns
    }

    fn interpret_digits(&self, digit_meanings: HashMap<u8, SegmentSet>) -> Vec<u8> {
        let mut answer = Vec::new();
        for digit in &self.output_value {
            for (digit_value, digit_pattern) in digit_meanings.iter() {
                if digit_pattern == digit {
                    answer.push(*digit_value);
                }
            }
        }
        answer
    }

    fn get_segment_frequency(
        chars_by_count: &HashMap<usize, Vec<char>>,
        count: usize,
    ) -> SegmentSet {
        chars_by_count
            .get(&count)
            .unwrap()
            .iter()
            .cloned()
            .collect()
    }

    fn get_unique_segment_frequency(
        chars_by_count: &HashMap<usize, Vec<char>>,
        count: usize,
    ) -> char {
        chars_by_count
            .get(&count)
            .unwrap()
            .iter()
            .cloned()
            .next()
            .unwrap()
    }

    fn insert_digit_pattern(digits: &mut HashMap<u8, SegmentSet>, digit: u8, chars: &[char]) {
        digits.insert(digit, chars.iter().cloned().collect());
    }

    fn insert_known_segment_count_pattern(
        digits: &mut HashMap<u8, SegmentSet>,
        digit: u8,
        count: usize,
        signal_patterns_by_length: &HashMap<usize, &SegmentSet>,
    ) {
        digits.insert(
            digit,
            signal_patterns_by_length
                .get(&count)
                .cloned()
                .unwrap()
                .clone(),
        );
    }
}

fn sum_digits(digits: Vec<u8>) -> u32 {
    let total_digits = digits.len();
    digits
        .into_iter()
        .enumerate()
        .map(|(pos, val)| val as u32 * 10u32.pow((total_digits - pos - 1) as u32))
        .sum()
}

fn main() {
    let input = BIG_INPUT.lines().map(Entry::new);
    let accumulator: u32 = input
        .map(|e| {
            let digits = e.deduce_digits();
            let outputs = e.interpret_digits(digits);
            sum_digits(outputs)
        })
        .sum();
    println!("Grand total {}", accumulator);
}
