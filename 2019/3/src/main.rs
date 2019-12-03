use std::cmp::Ord;
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
        "Part 1: Min distance: {}",
        min_distance(path_texts[0].as_str(), path_texts[1].as_str())
    );
    println!(
        "Part 2: Min steps: {}",
        min_steps(path_texts[0].as_str(), path_texts[1].as_str())
    );
}

fn min_distance(path0: &str, path1: &str) -> usize {
    let grid = initialize_grid(path0, path1);
    let mut min = usize::max_value();
    for (position, wires) in &grid.points {
        let distance = position.manhattan_distance();
        if wires.both_present() && distance < min && distance > 0 {
            min = distance;
        }
    }
    return min;
}

fn min_steps(path0: &str, path1: &str) -> usize {
    let grid = initialize_grid(path0, path1);
    let mut min = usize::max_value();
    for (_, wires) in &grid.points {
        if let Some(steps) = wires.total_steps() {
            if steps > 0 && steps < min {
                min = steps;
            }
        }
    }
    return min;
}

fn initialize_grid(path0: &str, path1: &str) -> Grid {
    let path0 = Path::parse(path0);
    let path1 = Path::parse(path1);
    let mut grid = Grid::new();
    path0.follow(&mut grid, 0);
    path1.follow(&mut grid, 1);
    return grid;
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

    fn move1(&self, position: &mut Point, step_count: &mut usize) {
        *step_count += 1;
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

    fn follow(&self, position: &mut Point, grid: &mut Grid, wirenum: u8, step_count: &mut usize) {
        grid.set(*position, wirenum, *step_count);
        for _ in 0..self.distance {
            self.direction.move1(position, step_count);
            grid.set(*position, wirenum, *step_count);
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
        let mut step_count: usize = 0;
        for step in &self.steps {
            step.follow(&mut position, grid, wirenum, &mut step_count);
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

fn set_min(steps: &mut Option<usize>, new_steps: usize) {
    *steps = steps.map_or(Some(new_steps), |old| Some(old.min(new_steps)));
}

struct WiresPresent {
    steps0: Option<usize>,
    steps1: Option<usize>,
}
impl WiresPresent {
    fn new() -> WiresPresent {
        WiresPresent {
            steps0: None,
            steps1: None,
        }
    }
    fn set(&mut self, wirenum: u8, steps: usize) {
        match wirenum {
            0 => set_min(&mut self.steps0, steps),
            1 => set_min(&mut self.steps1, steps),
            _ => panic!("Unexpected wire number: {}", wirenum),
        }
    }
    fn both_present(&self) -> bool {
        return self.steps0.is_some() && self.steps1.is_some();
    }
    fn total_steps(&self) -> Option<usize> {
        self.steps0
            .and_then(|steps0| self.steps1.map(|steps1| steps0 + steps1))
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

    fn set(&mut self, position: Point, wirenum: u8, step_count: usize) {
        self.points
            .entry(position)
            .or_insert(WiresPresent::new())
            .set(wirenum, step_count);
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
            min_steps(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            610
        );
        assert_eq!(
            min_distance(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            135
        );
        assert_eq!(
            min_steps(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            410
        );
    }
}
