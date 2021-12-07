static DEMO_INPUTS: &str = "16,1,2,0,4,2,7,1,2,14";

fn parse_crabs() -> Vec<isize> {
    DEMO_INPUTS
        .split(',')
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

fn main_part_one() {
    let crab_positions = parse_crabs();
    let min_pos = *crab_positions.iter().min().unwrap();
    let max_pos = *crab_positions.iter().max().unwrap();
    let mut best_pos_and_fuel = None;
    for pos in min_pos..=max_pos {
        let fuel: isize = crab_positions
            .iter()
            .cloned()
            .map(|crab_pos| (pos - crab_pos).abs())
            .sum();
        let pos_and_fuel = (pos, fuel);
        best_pos_and_fuel = Some(match best_pos_and_fuel {
            None => pos_and_fuel,
            Some((_, prev_fuel)) if prev_fuel > fuel => pos_and_fuel,
            Some(former) => former,
        })
    }
    println!("Fuel is {}", best_pos_and_fuel.unwrap().1);
}

fn fuel(distance: isize) -> isize {
    let mut accumulator = 0isize;
    for step in 0..=distance {
        accumulator += step
    }
    accumulator
}

fn main() {
    let crab_positions = parse_crabs();
    let min_pos = *crab_positions.iter().min().unwrap();
    let max_pos = *crab_positions.iter().max().unwrap();
    let mut best_pos_and_fuel = None;
    for pos in min_pos..=max_pos {
        let fuel: isize = crab_positions
            .iter()
            .cloned()
            .map(|crab_pos| fuel((pos - crab_pos).abs()))
            .sum();
        let pos_and_fuel = (pos, fuel);
        best_pos_and_fuel = Some(match best_pos_and_fuel {
            None => pos_and_fuel,
            Some((_, prev_fuel)) if prev_fuel > fuel => pos_and_fuel,
            Some(former) => former,
        })
    }
    println!("Fuel is {}", best_pos_and_fuel.unwrap().1);
}
