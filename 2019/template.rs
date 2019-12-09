use std::io;
use std::io::BufRead;

fn main() {
    let input = input();

    for _line in input.lines() {}

    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read input");
    input
}

fn part1(_: &Vec<String>) -> String {
    "".to_string()
}

fn part2(_: &Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!(part1(&vec![]), "");
        assert_eq!(part2(&vec![]), "");
    }
}
