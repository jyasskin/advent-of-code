use ndarray;
use num_integer::Integer;
use std::io;
use std::io::Read;

fn main() {
    let input = input();

    println!("Part 1: {:?}", locate_asteroid_detector(&input));
}

fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read input");
    input
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: isize,
    y: isize,
}
fn point(x: isize, y: isize) -> Point {
    Point { x, y }
}

fn locate_asteroid_detector(map: &str) -> (Point, i32) {
    let map = parse_map(map);
    let mut max_seen = 0;
    let mut max_pos = point(0, 0);
    for row in 0..map.nrows() {
        for col in 0..map.ncols() {
            if map[(row, col)] == 1 {
                // Can watch from an asteroid.
                let seen = compute_seen(row, col, &map);
                if seen > max_seen {
                    max_seen = seen;
                    max_pos = point(col as isize, row as isize);
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

fn ints_between(mut from: isize, to: isize, step: Option<isize>) -> Vec<isize> {
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

fn points_between(from: Point, to: Point) -> Vec<Point> {
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
                for Point { x, y } in points_between(
                    point(col as isize, row as isize),
                    point(test_col as isize, test_row as isize),
                ) {
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

    #[test]
    fn examples() {
        assert_eq!(
            locate_asteroid_detector(
                ".#..#
.....
#####
....#
...##"
            ),
            (point(3, 4), 8)
        );
        assert_eq!(
            locate_asteroid_detector(
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
            locate_asteroid_detector(
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
            locate_asteroid_detector(
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
            locate_asteroid_detector(
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
}
