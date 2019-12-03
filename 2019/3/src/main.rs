use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let path_texts: io::Result<Vec<String>> = io::stdin().lock().lines().collect();
    let path_texts = path_texts.expect("Couldn't read input");
    if path_texts.len() != 2 {
        panic!("Invalid input: {:?}", path_texts);
    }
    println!(
        "Part 1: {}",
        min_distance(path_texts[0].as_str(), path_texts[1].as_str())
    );
}

fn min_distance(path0: &str, path1: &str) -> usize {
    let path0 = Path::parse(path0);
    let path1 = Path::parse(path1);
    let mut grid = Grid::new();
    path0.follow(&mut grid, 0);
    path1.follow(&mut grid, 1);
    let mut min = usize::max_value();
    for (position, wires) in &grid.points {
        let distance = position.manhattan_distance();
        if wires.both_present() && distance < min && distance > 0 {
            min = distance;
        }
    }
    return min;
}

enum Direction {
    Right,
    Down,
    Left,
    Up,
}
impl Direction {
    fn parse(input: char) -> Direction {
        match input {
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'U' => Direction::Up,
            _ => panic!("Unexpected direction: {}", input),
        }
    }

    fn move1(&self, position: &mut Point) {
        match &self {
            Direction::Right => position.x += 1,
            Direction::Down => position.y -= 1,
            Direction::Left => position.x -= 1,
            Direction::Up => position.y += 1,
        }
    }
}
struct Step {
    direction: Direction,
    distance: usize,
}
impl Step {
    fn parse(input: &str) -> Step {
        let mut input: String = input.to_string();
        let direction = Direction::parse(input.remove(0));
        let distance: usize = input.parse().expect("Expected an integer in step");
        return Step {
            direction: direction,
            distance: distance,
        };
    }

    fn follow(&self, position: &mut Point, grid: &mut Grid, wirenum: u8) {
        grid.set(*position, wirenum);
        for _ in 0..self.distance {
            self.direction.move1(position);
            grid.set(*position, wirenum);
        }
    }
}

struct Path {
    steps: Vec<Step>,
}
impl Path {
    fn parse(input: &str) -> Path {
        let steps = input.split(',');
        return Path {
            steps: steps.map(Step::parse).collect(),
        };
    }

    fn follow(&self, grid: &mut Grid, wirenum: u8) {
        let mut position = Point { x: 0, y: 0 };
        for step in &self.steps {
            step.follow(&mut position, grid, wirenum);
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}
impl Point {
    fn manhattan_distance(&self) -> usize {
        return (self.x.abs() + self.y.abs()) as usize;
    }
}

struct WiresPresent {
    bits: u8,
}
impl WiresPresent {
    fn new() -> WiresPresent {
        WiresPresent { bits: 0 }
    }
    fn set(&mut self, wirenum: u8) {
        self.bits |= 1 << wirenum;
    }
    fn both_present(&self) -> bool {
        return self.bits == 3;
    }
}

struct Grid {
    points: HashMap<Point, WiresPresent>,
}
impl Grid {
    fn new() -> Grid {
        Grid {
            points: HashMap::new(),
        }
    }

    fn set(&mut self, position: Point, wirenum: u8) {
        self.points
            .entry(position)
            .or_insert(WiresPresent::new())
            .set(wirenum);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!(
            min_distance(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            159
        );
        assert_eq!(
            min_distance(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
    }
}
