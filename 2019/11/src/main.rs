use euclid::*;
use intcode;
use std::collections::HashMap;
use std::io;
use std::io::Read;

type Point = default::Point2D<isize>;

fn main() {
    let input = input();

    let program: Vec<i64> = input
        .split(',')
        .map(|s| s.trim().parse().expect(s))
        .collect();

    let mut state = State {
        position: Point::new(0, 0),
        heading: Direction::Up,
        painted: HashMap::new(),
        relative_base: 0,
        next_output: NextOutput::Paint,
    };
    intcode::run_intcode(program, &mut state);

    println!("Part 1: {}", state.painted.len());
    println!("Part 2: {}", "");
}

const BLACK: i64 = 0;
// const WHITE: i64 = 1;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn step(self, p: Point) -> Point {
        match self {
            Direction::Up => Point::new(p.x, p.y + 1),
            Direction::Down => Point::new(p.x, p.y - 1),
            Direction::Left => Point::new(p.x - 1, p.y),
            Direction::Right => Point::new(p.x + 1, p.y),
        }
    }
    fn turn(self, arg: i64) -> Direction {
        match arg {
            0 => {
                // Left
                match self {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                }
            }
            1 => {
                // Right
                match self {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                }
            }
            _ => panic!("Unexpected turn direction: {}", arg),
        }
    }
}
struct State {
    position: Point,
    heading: Direction,
    painted: HashMap<Point, i64>,
    relative_base: i64,
    next_output: NextOutput,
}
#[derive(Copy, Clone)]
enum NextOutput {
    Paint,
    Turn,
}

impl intcode::State for State {
    fn input(&mut self) -> Option<i64> {
        Some(match self.painted.get(&self.position) {
            None => BLACK,
            Some(c) => *c,
        })
    }
    fn output(&mut self, val: i64) {
        match self.next_output {
            NextOutput::Paint => {
                self.next_output = NextOutput::Turn;
                self.painted.insert(self.position, val);
            }
            NextOutput::Turn => {
                self.next_output = NextOutput::Paint;
                self.heading = self.heading.turn(val);
                self.position = self.heading.step(self.position);
            }
        }
    }

    fn copy_output(&self) -> Vec<i64> {
        vec![]
    }
    fn adjust_relative_base(&mut self, adj: i64) {
        self.relative_base += adj;
    }
    fn relative_base(&self) -> i64 {
        self.relative_base
    }
}

fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read input");
    input
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!("", "");
        assert_eq!("", "");
    }
}
