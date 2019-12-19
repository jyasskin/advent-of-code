use advent_util::*;

fn main() {
    let input = input();

    let program: Vec<i64> = input
        .split(',')
        .map(|s| s.trim().parse().expect(s))
        .collect();

    let mut tractor_area: i64 = 0;
    for x in 0..50 {
        for y in 0..50 {
            let result = intcode::run_intcode_input(program.clone(), &[x, y]);
            tractor_area += result.output[0];
        }
    }

    println!("Part 1: {}", tractor_area);
    println!("Part 2: {}", part2(&input.lines().collect()));
}

fn part2(_: &Vec<&str>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!(part2(&vec![]), "");
    }
}
