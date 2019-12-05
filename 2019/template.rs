use std::io;
use std::io::BufRead;

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .expect("Couldn't read input");
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
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
