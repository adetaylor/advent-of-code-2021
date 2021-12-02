static INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

fn main_old() {
    let steps = INPUT.split('\n');
    let mut pos = 0;
    let mut depth = 0;
    for s in steps {
        let (cmd, distance) = s.split_once(' ').unwrap();
        let distance = str::parse::<i32>(distance).unwrap();
        let (dpos, ddepth) = match cmd {
            "forward" => (distance, 0),
            "down" => (0, distance),
            "up" => (0, -distance),
            _ => panic!("unexpected command"),
        };
        pos += dpos;
        depth += ddepth;
    }
    println!("hpos: {}, depth: {}, product {}", pos, depth, pos*depth);
}

fn main() {
    let steps = INPUT.split('\n');
    let mut pos = 0i64;
    let mut depth = 0i64;
    let mut aim = 0i64;
    for s in steps {
        let (cmd, distance) = s.split_once(' ').unwrap();
        let distance = str::parse::<i64>(distance).unwrap();
        let (dpos, ddepth, daim) = match cmd {
            "forward" => (distance, distance*aim, 0),
            "down" => (0, 0, distance),
            "up" => (0, 0, -distance),
            _ => panic!("unexpected command"),
        };
        aim += daim;
        pos += dpos;
        depth += ddepth;
    }
    println!("hpos: {}, depth: {}, product {}", pos, depth, pos*depth);
}
