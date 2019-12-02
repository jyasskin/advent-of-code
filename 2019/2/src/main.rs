fn main() {
    let initial_memory = vec![
        1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 19, 5, 23, 1, 6, 23, 27,
        1, 27, 5, 31, 2, 31, 10, 35, 2, 35, 6, 39, 1, 39, 5, 43, 2, 43, 9, 47, 1, 47, 6, 51, 1, 13,
        51, 55, 2, 9, 55, 59, 1, 59, 13, 63, 1, 6, 63, 67, 2, 67, 10, 71, 1, 9, 71, 75, 2, 75, 6,
        79, 1, 79, 5, 83, 1, 83, 5, 87, 2, 9, 87, 91, 2, 9, 91, 95, 1, 95, 10, 99, 1, 9, 99, 103,
        2, 103, 6, 107, 2, 9, 107, 111, 1, 111, 5, 115, 2, 6, 115, 119, 1, 5, 119, 123, 1, 123, 2,
        127, 1, 127, 9, 0, 99, 2, 0, 14, 0,
    ];
    println!("Part 1: {:?}", run_with_args(initial_memory.clone(), 12, 2));
    for noun in 0..100 {
        for verb in 0..100 {
            if run_with_args(initial_memory.clone(), noun, verb)[0] == 19690720 {
                println!("{}, {}", noun, verb);
            }
        }
    }
}

fn run_with_args(mut memory: Vec<i32>, noun: i32, verb: i32) -> Vec<i32> {
    memory[1] = noun;
    memory[2] = verb;
    return run_intcode(memory);
}

fn run_intcode(mut program: Vec<i32>) -> Vec<i32> {
    let mut position = 0;
    loop {
        match program[position] {
            1 => {
                let arg1_pos = program[position + 1] as usize;
                let arg2_pos = program[position + 2] as usize;
                let result_pos = program[position + 3] as usize;
                program[result_pos] = program[arg1_pos] + program[arg2_pos];
            }
            2 => {
                let arg1_pos = program[position + 1] as usize;
                let arg2_pos = program[position + 2] as usize;
                let result_pos = program[position + 3] as usize;
                program[result_pos] = program[arg1_pos] * program[arg2_pos];
            }
            99 => return program,
            _ => panic!(
                "Unexpected opcode at position {}: {}",
                position, program[position]
            ),
        }
        position += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!(
            run_intcode(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert_eq!(run_intcode(vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(run_intcode(vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(
            run_intcode(vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            run_intcode(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
