use advent_util::*;

fn main() {
    let input = input();

    let program: Vec<i64> = input
        .split(',')
        .map(|s| s.trim().parse().expect(s))
        .collect();

    println!("Part 1: {}", part1(&program));
    let ship = part2(&program);
    println!(
        "Part 2: {}; {}",
        ship,
        ship.origin.x * 10000 + ship.origin.y
    );
}

fn part1(program: &Vec<i64>) -> i64 {
    let mut tractor_area: i64 = 0;
    for x in 0..50 {
        for y in 0..50 {
            let result = intcode::run_intcode_input(program.clone(), &[x, y]);
            tractor_area += result.output[0];
        }
    }
    tractor_area
}

fn test_point(p: Point2, program: &Vec<i64>) -> bool {
    let result = intcode::run_intcode_input(program.clone(), &[p.x, p.y]);
    result.output[0] == 1
}

fn bottom_left(r: &Rect) -> Point2 {
    point2(r.min_x(), r.max_y())
}
fn top_right(r: &Rect) -> Point2 {
    point2(r.max_x(), r.min_y())
}

fn part2(program: &Vec<i64>) -> Rect {
    let mut ship = Rect::new(point2(1, 1), size2(99, 99));
    let mut fits = false;
    while !fits {
        fits = true;
        if !test_point(bottom_left(&ship), program) {
            fits = false;
            ship = ship.translate(vec2(1, 0));
        }
        if !test_point(top_right(&ship), program) {
            fits = false;
            ship = ship.translate(vec2(0, 1));
        }
    }
    ship
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!(part2(&vec![]), "");
    }
}
