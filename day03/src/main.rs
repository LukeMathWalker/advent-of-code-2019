use std::io::BufRead;
use std::str::FromStr;
use std::collections::HashSet;

type GridPoint = (i64, i64);

#[derive(Clone)]
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
            _ => panic!("Unknown direction - failed to parse."),
        }
    }

    fn move_unit(&self) -> Box<dyn Fn(GridPoint) -> GridPoint>
    {
        match self {
            Direction::Up => Box::new(|p: GridPoint| (p.0 + 1, p.1)),
            Direction::Down => Box::new(|p: GridPoint| (p.0 - 1, p.1)),
            Direction::Right => Box::new(|p: GridPoint| (p.0, p.1 + 1)),
            Direction::Left => Box::new(|p: GridPoint| (p.0, p.1 - 1)),
        }
    }
}

#[derive(Clone)]
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

fn get_grid_points(wire: &[Movement]) -> HashSet<GridPoint> {
    let mut current_position = (0, 0);

    let mut grid_points = HashSet::new();

    for movement in wire {
        let step_function = movement.direction.move_unit();
        for _ in 0..movement.distance {
            current_position = step_function(current_position);
            grid_points.insert(current_position.clone());
        }
    }
    grid_points
}

fn main() {
    let movements = read_input("input.txt");
    assert_eq!(movements.len(), 2);
    let (first_wire, second_wire) = (movements[0].clone(), movements[1].clone());
    let first_grid_points = get_grid_points(&first_wire);
    let second_grid_points = get_grid_points(&second_wire);
    let common_points = first_grid_points.intersection(&second_grid_points);
    let closest_point = common_points.into_iter().min_by(|p, q| (p.0.abs() + p.1.abs()).cmp(&(q.0.abs() + q.1.abs()))).unwrap();
    println!("Closest point: {:?}", closest_point);
}
