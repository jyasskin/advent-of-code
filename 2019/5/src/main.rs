use std::io;
use std::io::BufRead;

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .expect("Couldn't read input");
    let IntcodeResult { output, .. } = run_intcode(
        lines[0]
            .split(',')
            .map(|s| s.parse().expect("Must be an integer"))
            .collect(),
        vec![1],
    );
    println!("Part 1: {:?}", output);
    println!("Part 2: {}", part2(&lines));
}

#[derive(Clone, Copy)]
enum Opcode {
    Add,
    Mul,
    In,
    Out,
    Halt,
}
impl Opcode {
    fn new(code: i32) -> Opcode {
        match code {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::In,
            4 => Opcode::Out,
            99 => Opcode::Halt,
            _ => panic!("Unexpected opcode: {}", code),
        }
    }
    fn length(self) -> usize {
        match self {
            Opcode::Add => 4,
            Opcode::Mul => 4,
            Opcode::In => 2,
            Opcode::Out => 2,
            Opcode::Halt => 1,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct IntcodeResult {
    tape: Vec<i32>,
    output: Vec<i32>,
}

fn run_intcode(mut program: Vec<i32>, input: Vec<i32>) -> IntcodeResult {
    let mut position = 0;
    let mut input = input.into_iter();
    let mut output = Vec::<i32>::new();
    loop {
        let opcode = Opcode::new(program[position] % 100);
        let mut allmodes = program[position] / 100;
        let mut modes: Vec<i32> = vec![];
        while allmodes != 0 {
            modes.push(allmodes % 10);
            allmodes /= 10;
        }
        match opcode {
            Opcode::Add => {
                modes.resize(2, 0);
                let mut arg0 = program[position + 1];
                if modes[0] == 0 {
                    arg0 = program[arg0 as usize];
                }
                let mut arg1 = program[position + 2];
                if modes[1] == 0 {
                    arg1 = program[arg1 as usize];
                }
                let result_pos = program[position + 3];
                program[result_pos as usize] = arg0 + arg1;
            }
            Opcode::Mul => {
                modes.resize(2, 0);
                let mut arg0 = program[position + 1];
                if modes[0] == 0 {
                    arg0 = program[arg0 as usize];
                }
                let mut arg1 = program[position + 2];
                if modes[1] == 0 {
                    arg1 = program[arg1 as usize];
                }
                let result_pos = program[position + 3];
                program[result_pos as usize] = arg0 * arg1;
            }
            Opcode::In => {
                let result_pos = program[position + 1];
                program[result_pos as usize] = input.next().expect("Not enough input");
            }
            Opcode::Out => {
                modes.resize(1, 0);
                let mut arg0 = program[position + 1];
                if modes[0] == 0 {
                    arg0 = program[arg0 as usize];
                }
                output.push(arg0);
            }
            Opcode::Halt => {
                return IntcodeResult {
                    tape: program,
                    output: output,
                }
            }
        }
        position += opcode.length();
    }
}

fn part2(_: &Vec<String>) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!(
            run_intcode(vec![1002, 4, 3, 4, 33], vec![]),
            IntcodeResult {
                tape: vec![1002, 4, 3, 4, 99],
                output: vec![]
            }
        );
        //assert_eq!(part2(&vec![]), "");
    }
}
