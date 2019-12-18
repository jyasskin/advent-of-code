use advent_util::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fmt::Display;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    let input = read_file(&env::args().nth(1).unwrap());

    let mut program: Vec<i64> = input
        .split(',')
        .map(|s| s.trim().parse().expect(s))
        .collect();

    let result = intcode::run_intcode_input(program.clone(), &[]);

    println!(
        "Part 1: {}",
        result
            .output
            .chunks_exact(3)
            .filter(|tile| tile[2] == 2)
            .count()
    );

    program[0] = 2;

    println!("Part 2: {}", part2(program));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,   // Walls are indestructible barriers.
    Block,  // Blocks can be broken by the ball.
    Paddle, // The paddle is indestructible.
    Ball,   // The ball moves diagonally and bounces off objects.
}
fn tile(i: i64) -> Tile {
    match i {
        0 => Tile::Empty,
        1 => Tile::Wall,
        2 => Tile::Block,
        3 => Tile::Paddle,
        4 => Tile::Ball,
        _ => panic!("Invalid tile: {}", i),
    }
}
impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            Tile::Empty => " ",
            Tile::Wall => "#",
            Tile::Block => "X",
            Tile::Paddle => "_",
            Tile::Ball => "o",
        })
    }
}

#[derive(Default)]
struct Game {
    next_tile: Vec<i64>,
    relative_base: i64,
    display: HashMap<Point2, Tile>,
    ball: Point2,
    ball_direction: Vector2,
    paddle: Point2,
    score: i64,
}
impl Game {
    fn update_ball(&mut self, new_ball: Point2) {
        self.ball_direction = new_ball - self.ball;
        self.ball = new_ball;
    }
    fn print(&self) {
        let bounds = Rect::from_points(self.display.keys());
        for y in bounds.min_y()..=bounds.max_y() {
            for x in bounds.min_x()..=bounds.max_x() {
                print!(
                    "{}",
                    self.display.get(&point2(x, y)).unwrap_or(&Tile::Empty)
                );
            }
            println!();
        }
        println!("Score: {}", self.score);
    }
}

fn _read_paddle() -> i64 {
    let stdin = io::stdin();
    loop {
        println!("a: left; s: stop; d: right: ");
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        match line.trim() {
            "a" => break -1,
            "s" => break 0,
            "d" => break 1,
            _ => {}
        }
    }
}

impl intcode::State for Game {
    fn input(&mut self) -> Option<i64> {
        thread::sleep(Duration::from_millis(1));
        self.print();
        //Some(read_paddle())
        Some(match self.paddle.x.cmp(&self.ball.x) {
            Ordering::Less => {
                if self.ball_direction.x < 0 {
                    0
                } else {
                    1
                }
            }
            Ordering::Equal => {
                if (self.paddle - self.ball).y > 1 {
                    self.ball_direction.x
                } else {
                    0
                }
            }
            Ordering::Greater => {
                if self.ball_direction.x > 0 {
                    0
                } else {
                    -1
                }
            }
        })
    }
    fn output(&mut self, out: i64) {
        self.next_tile.push(out);
        if self.next_tile.len() == 3 {
            let coords = point2(self.next_tile[0], self.next_tile[1]);
            if coords == point2(-1, 0) {
                self.score = self.next_tile[2];
            } else {
                let t = tile(self.next_tile[2]);
                self.display.insert(coords, t);
                match t {
                    Tile::Ball => self.update_ball(coords),
                    Tile::Paddle => self.paddle = coords,
                    _ => {}
                }
            }
            self.next_tile.clear();
        }
    }
    fn copy_output(&self) -> Vec<i64> {
        vec![self.score]
    }
    fn adjust_relative_base(&mut self, adj: i64) {
        self.relative_base += adj;
    }
    fn relative_base(&self) -> i64 {
        self.relative_base
    }
}

// Returns the score
fn part2(program: Vec<i64>) -> i64 {
    let mut game: Game = Default::default();
    let result = intcode::run_intcode(program.clone(), &mut game);
    result.output[0]
}
