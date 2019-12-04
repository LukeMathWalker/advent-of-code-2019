use std::io::BufRead;
use std::str::FromStr;

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn parse(s: &str) -> Self {
        match s {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
            "L" => Direction::Left,
            s => {
                println!("{}", s);
                panic!("Unknown direction - failed to parse.")
            },
        }
    }
}

struct Movement {
    direction: Direction,
    distance: u64,
}

impl Movement {
    fn parse(s: &str) -> Self {
        let direction = Direction::parse(&s[0..1]);
        let distance = u64::from_str(&s[1..]).expect("Failed to parse distance.");
        Self {
            direction,
            distance,
        }
    }
}

fn read_input(path: &str) -> Vec<Vec<Movement>> {
    let file = std::fs::File::open(path).expect("Failed to open file.");
    let reader = std::io::BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            let line = l.expect("Failed to read line");
            line.split(",").map(Movement::parse).collect()
        })
        .collect()
}

fn main() {
    let movements = read_input("input.txt");
    assert_eq!(movements.len(), 2);
}
