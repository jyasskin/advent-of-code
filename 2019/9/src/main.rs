use intcode::*;
use std::io;
use std::io::Read;

fn main() {
    let input = input();

    println!(
        "Part 1: {:?}",
        run_intcode_input(
            input
                .split(',')
                .map(|s| s
                    .trim()
                    .parse()
                    .expect(&["Must be an integer: ", s].concat()))
                .collect(),
            &[1]
        )
        .output
    );
    println!(
        "Part 1: {:?}",
        run_intcode_input(
            input
                .split(',')
                .map(|s| s
                    .trim()
                    .parse()
                    .expect(&["Must be an integer: ", s].concat()))
                .collect(),
            &[2]
        )
        .output
    );
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
    }
}
