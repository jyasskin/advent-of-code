use advent_util::*;
use ndarray;
use num::Integer;
use std::convert::TryFrom;
use std::io;
use std::io::Read;

fn main() {
    let input = input();

    let map = parse_map(&input);

    let (station, seen) = locate_asteroid_detector(&map);
    println!("Part 1: {:?}", (station, seen));
    println!("Part 2: {:?}", vaporization_order(&station, map)[199]);
}

fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read input");
    input
}

type Point2 = euclid::default::Point2D<i32>;
type Vector2 = euclid::default::Vector2D<i32>;
fn point(x: i32, y: i32) -> Point2 {
    Point2::new(x, y)
}

fn print_map(map: &ndarray::Array2<i32>, laser: Option<&Point2>) {
    for row in 0..map.nrows() {
        for col in 0..map.ncols() {
            print!(
                "{}",
                if Some(&point(col as i32, row as i32)) == laser {
                    "*"
                } else if map[(row, col)] == 1 {
                    "#"
                } else {
                    " "
                }
            );
        }
        println!();
    }
    println!();
}

fn locate_asteroid_detector(map: &ndarray::Array2<i32>) -> (Point2, i32) {
    let mut max_seen = 0;
    let mut max_pos = point(0, 0);
    for row in 0..map.nrows() {
        for col in 0..map.ncols() {
            if map[(row, col)] == 1 {
                // Can watch from an asteroid.
                let seen = compute_seen(row, col, &map);
                if seen > max_seen {
                    max_seen = seen;
                    max_pos = point(col as i32, row as i32);
                }
            }
        }
    }
    return (max_pos, max_seen);
}

fn parse_map(s: &str) -> ndarray::Array2<i32> {
    let rows: Vec<Vec<i32>> = s
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => 1,
                    '.' => 0,
                    _ => panic!("Unexpected input: {}", c),
                })
                .collect()
        })
        .collect();
    ndarray::Array2::from_shape_fn((rows.len(), rows[0].len()), |(row, col)| rows[row][col])
}

fn ints_between(mut from: i32, to: i32, step: Option<i32>) -> Vec<i32> {
    let step = match step {
        Some(step) => step,
        None => {
            if from < to {
                1
            } else {
                -1
            }
        }
    };
    let mut result = vec![];
    from += step;
    while from * step < to * step {
        result.push(from);
        from += step;
    }
    result
}

fn points_between(from: Point2, to: Point2) -> Vec<Point2> {
    if from.x == to.x {
        return ints_between(from.y, to.y, None)
            .iter()
            .map(|y| point(from.x, *y))
            .collect();
    } else if from.y == to.y {
        return ints_between(from.x, to.x, None)
            .iter()
            .map(|x| point(*x, from.y))
            .collect();
    } else {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        let gcd = dx.gcd(&dy);
        ints_between(from.x, to.x, Some(dx / gcd))
            .into_iter()
            .zip(ints_between(from.y, to.y, Some(dy / gcd)))
            .map(|(x, y)| point(x, y))
            .collect()
    }
}

fn compute_seen(row: usize, col: usize, map: &ndarray::Array2<i32>) -> i32 {
    let mut num_seen = 0;
    for test_row in 0..map.nrows() {
        'asteroids: for test_col in 0..map.ncols() {
            if map[(test_row, test_col)] == 1 && (test_row != row || test_col != col) {
                for (x, y) in points_between(
                    point(col as i32, row as i32),
                    point(test_col as i32, test_row as i32),
                )
                .into_iter()
                .map(|p| p.to_tuple())
                {
                    if map[(y as usize, x as usize)] == 1 {
                        continue 'asteroids;
                    }
                }
                num_seen += 1;
            }
        }
    }
    num_seen
}

fn vaporization_order(laser: &Point2, mut map: ndarray::Array2<i32>) -> Vec<Point2> {
    let max_distance: i32 = *[
        laser.x,
        laser.y,
        map.nrows() as i32 - laser.y,
        map.ncols() as i32 - laser.x,
    ]
    .iter()
    .max()
    .unwrap();
    let angles: Vec<Vector2> = stern_brocot_sequence(max_distance)
        .iter()
        .map(|angle| Vector2::new(*angle.numer(), *angle.denom()))
        .filter(|angle| angle.x <= max_distance && angle.y <= max_distance)
        .collect();
    let mut result = vec![];
    while map.sum() > 1 {
        if let Some(asteroid) = pew(&Vector2::new(0, -1), laser, &mut map) {
            result.push(asteroid);
        }
        // Top-right quadrant.
        for angle in angles.iter() {
            if let Some(asteroid) = pew(&Vector2::new(angle.x, -angle.y), laser, &mut map) {
                result.push(asteroid);
            }
        }
        if let Some(asteroid) = pew(&Vector2::new(1, 0), laser, &mut map) {
            result.push(asteroid);
        }
        // Bottom-right quadrant.
        for angle in angles.iter().rev() {
            if let Some(asteroid) = pew(&Vector2::new(angle.x, angle.y), laser, &mut map) {
                result.push(asteroid);
            }
        }
        if let Some(asteroid) = pew(&Vector2::new(0, 1), laser, &mut map) {
            result.push(asteroid);
        }
        // Bottom-left quadrant.
        for angle in angles.iter() {
            if let Some(asteroid) = pew(&Vector2::new(-angle.x, angle.y), laser, &mut map) {
                result.push(asteroid);
            }
        }
        if let Some(asteroid) = pew(&Vector2::new(-1, 0), laser, &mut map) {
            result.push(asteroid);
        }
        // Top-left quadrant.
        for angle in angles.iter().rev() {
            if let Some(asteroid) = pew(&Vector2::new(-angle.x, -angle.y), laser, &mut map) {
                result.push(asteroid);
            }
        }
    }
    result
}

fn pew(angle: &Vector2, laser: &Point2, map: &mut ndarray::Array2<i32>) -> Option<Point2> {
    for multiple in multiples_to(angle, map.nrows().max(map.ncols()) as i32 + 1) {
        let try_asteroid = *laser + multiple;
        if try_asteroid.x < 0 || try_asteroid.y < 0 {
            break;
        }
        match map.get_mut((
            usize::try_from(try_asteroid.y).unwrap(),
            usize::try_from(try_asteroid.x).unwrap(),
        )) {
            None => break,
            Some(point) => {
                if *point > 0 {
                    *point = 0;
                    return Some(try_asteroid);
                }
            }
        }
    }
    None
}

fn multiples_to(v: &Vector2, max: i32) -> Vec<Vector2> {
    let mut result = vec![];
    for mul in 1.. {
        let next = Vector2::new(v.x * mul, v.y * mul);
        if next.x.abs() > max || next.y.abs() > max {
            break;
        }
        result.push(next);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ints_between() {
        assert_eq!(ints_between(0, 5, None), vec![1, 2, 3, 4]);
        assert_eq!(ints_between(5, 0, None), vec![4, 3, 2, 1]);
        assert_eq!(ints_between(5, 0, Some(-2)), vec![3, 1]);
    }

    #[test]
    fn test_points_between() {
        assert_eq!(
            points_between(point(1, 1), point(1, 5)),
            vec![point(1, 2), point(1, 3), point(1, 4)]
        );
        assert_eq!(
            points_between(point(1, 5), point(1, 1)),
            vec![point(1, 4), point(1, 3), point(1, 2)]
        );
        assert_eq!(
            points_between(point(1, 1), point(4, 1)),
            vec![point(2, 1), point(3, 1)]
        );
        assert_eq!(
            points_between(point(4, 1), point(1, 1)),
            vec![point(3, 1), point(2, 1)]
        );

        assert_eq!(points_between(point(4, 1), point(1, 2)), vec![]);
        assert_eq!(
            points_between(point(1, 1), point(7, 10)),
            vec![point(3, 4), point(5, 7)]
        );
        assert_eq!(
            points_between(point(-3, 3), point(1, 1)),
            vec![point(-1, 2)]
        );
        assert_eq!(
            points_between(point(1, 1), point(-3, 3)),
            vec![point(-1, 2)]
        );
        assert_eq!(
            points_between(point(7, 10), point(1, 1)),
            vec![point(5, 7), point(3, 4)]
        );
    }

    fn test_locate(s: &str) -> (Point2, i32) {
        locate_asteroid_detector(&parse_map(s))
    }

    #[test]
    fn examples() {
        assert_eq!(
            test_locate(
                ".#..#
.....
#####
....#
...##"
            ),
            (point(3, 4), 8)
        );
        assert_eq!(
            test_locate(
                "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"
            ),
            (point(5, 8), 33)
        );
        assert_eq!(
            test_locate(
                "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."
            ),
            (point(1, 2), 35)
        );
        assert_eq!(
            test_locate(
                ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."
            ),
            (point(6, 3), 41)
        );
        assert_eq!(
            test_locate(
                ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"
            ),
            (point(11, 13), 210)
        );
    }

    #[test]
    fn test_vaporization() {
        let map = parse_map(
            ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
        );
        let order = vaporization_order(&point(11, 13), map);
        assert_eq!(order[0], point(11, 12));
        assert_eq!(order[1], point(12, 1));
        assert_eq!(order[2], point(12, 2));
        assert_eq!(order[9], point(12, 8));
        assert_eq!(order[19], point(16, 0));
        assert_eq!(order[49], point(16, 9));
        assert_eq!(order[99], point(10, 16));
        assert_eq!(order[198], point(9, 6));
        assert_eq!(order[199], point(8, 2));
        assert_eq!(order[200], point(10, 9));
        assert_eq!(order[298], point(11, 1));
        assert_eq!(order.len(), 299);
    }
}
