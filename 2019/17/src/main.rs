use advent_util::Point2;
use intcode;
use std::char;
use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::str;

fn main() {
    let input = input();

    let mut program: Vec<i64> = input
        .split(',')
        .map(|s| s.trim().parse().expect(s))
        .collect();

    let result = intcode::run_intcode_input(program.clone(), &[]);
    let scaffold: String = result
        .output
        .iter()
        .map(|c| char::from_u32(*c as u32).unwrap())
        .collect();
    println!("{}", scaffold);

    println!("Part 1: {}", part1(&scaffold));

    // Change to movement mode.
    program[0] = 2;

    // Total program =
    // L,12,L,6,L,8,R,6,
    // L,8,L,8,R,4,R,6,R,6,
    // L,12,L,6,L,8,R,6,
    // L,8,L,8,R,4,R,6,R,6,
    // L,12,R,6,L,8,
    // L,12,R,6,L,8,
    // L,8,L,8,R,4,R,6,R,6,
    // L,12,L,6,L,8,R,6,
    // L,8,L,8,R,4,R,6,R,6,
    // L,12,R,6,L,8

    // A=L,12,L,6,L,8,R,6,
    // B=L,8,L,8,R,4,R,6,R,6,
    // C=L,12,R,6,L,8,
    // Program=A,B,A,B,C,C,B,A,B,C

    let input = "A,B,A,B,C,C,B,A,B,C
L,12,L,6,L,8,R,6
L,8,L,8,R,4,R,6,R,6
L,12,R,6,L,8
n
";

    let result = intcode::run_intcode_input(
        program.clone(),
        &input.chars().map(|c| c as i64).collect::<Vec<i64>>(),
    );

    println!("Part 2: {:?}", result.output);
}

fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read input");
    input
}

#[derive(PartialEq, Eq, Debug)]
enum Item {
    Open,
    Scaffold,
    Up,
    Down,
    Left,
    Right,
    Tumbling,
    Newline,
}

fn parse_item(c: char) -> Item {
    match c {
        '.' => Item::Open,
        '#' => Item::Scaffold,
        '^' => Item::Up,
        '>' => Item::Right,
        '<' => Item::Left,
        'v' => Item::Down,
        'X' => Item::Tumbling,
        '\x0A' => Item::Newline,
        _ => panic!("Unexpected Item: {}", c),
    }
}

fn is_scaffold(i: &Item) -> bool {
    match i {
        Item::Open => false,
        Item::Scaffold => true,
        Item::Up => true,
        Item::Down => true,
        Item::Left => true,
        Item::Right => true,
        Item::Tumbling => false,
        Item::Newline => false,
    }
}

fn parse_scaffold(s: &str) -> HashMap<Point2, Item> {
    let mut cur = Point2::zero();
    let mut result = HashMap::new();
    for c in s.chars() {
        let item = parse_item(c);
        if item == Item::Newline {
            cur.y += 1;
            cur.x = 0;
            continue;
        }
        result.insert(cur, item);
        cur.x += 1;
    }
    result
}

fn part1(scaffold: &str) -> i64 {
    let scaffold = parse_scaffold(scaffold);
    let mut total_adjustment = 0;
    for (point, i) in scaffold.iter() {
        if is_scaffold(i) {
            if let Some(left) = scaffold.get(&Point2::new(point.x - 1, point.y)) {
                if let Some(up) = scaffold.get(&Point2::new(point.x, point.y - 1)) {
                    if let Some(right) = scaffold.get(&Point2::new(point.x + 1, point.y)) {
                        if let Some(down) = scaffold.get(&Point2::new(point.x, point.y + 1)) {
                            if is_scaffold(left)
                                && is_scaffold(up)
                                && is_scaffold(right)
                                && is_scaffold(down)
                            {
                                total_adjustment += point.x * point.y;
                            }
                        }
                    }
                }
            }
        }
    }
    total_adjustment
}

fn part2(_: &Vec<&str>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!(
            part1(
                "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^.."
            ),
            76
        );
        //assert_eq!(part2(&vec![]), "");
    }
}
