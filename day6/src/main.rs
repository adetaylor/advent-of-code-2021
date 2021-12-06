use std::collections::HashMap;

static DEMO_INPUT: &str = "3,4,3,1,2";

fn main() {
    let fish = DEMO_INPUT
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let mut fish_by_age: HashMap<u8, u64> = HashMap::new();
    for f in fish {
        *fish_by_age.entry(f).or_default() += 1;
    }
    println!("{:?}", fish_by_age);
    for _ in 1..257 {
        let day_zero_fish = *fish_by_age.entry(0u8).or_default();
        for age in 1..9 {
            *fish_by_age.entry(age - 1).or_default() = *fish_by_age.entry(age).or_default();
        }
        *fish_by_age.entry(6u8).or_default() += day_zero_fish;
        *fish_by_age.entry(8u8).or_default() = day_zero_fish;
        //println!("{:?}", fish_by_age);
    }
    println!("Total fish {}", fish_by_age.into_values().sum::<u64>());
}
