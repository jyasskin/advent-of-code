use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(PartialEq, Eq, Debug)]
pub struct IntcodeResult {
    pub memory: Vec<i32>,
    pub output: Vec<i32>,
}

pub fn run_intcode(mut program: Vec<i32>, io: &mut dyn IO) -> IntcodeResult {
    let mut position = 0;
    loop {
        let opcode = Opcode::new(program[position] % 100);
        let mut allmodes = program[position] / 100;
        let mut modes: Vec<i32> = vec![];
        while allmodes != 0 {
            modes.push(allmodes % 10);
            allmodes /= 10;
        }
        modes.resize(opcode.params(), 0);
        let modes: Vec<Mode> = modes.into_iter().map(Mode::new).collect();
        let params = Box::from(&program[position + 1..position + 1 + opcode.params()]);
        match opcode.execute(&params, &modes, &mut program, io) {
            OpcodeResult::Continue => position += opcode.params() + 1,
            OpcodeResult::JumpTo(target) => position = target,
            OpcodeResult::Halt => {
                return IntcodeResult {
                    memory: program,
                    output: io.copy_output(),
                }
            }
        }
    }
}

pub fn run_intcode_input(program: Vec<i32>, input: &[i32]) -> IntcodeResult {
    run_intcode(program, &mut VecIO::new(input.into()))
}

pub trait IO {
    fn read(&mut self) -> Option<i32>;
    fn write(&mut self, val: i32);
    fn copy_output(&self) -> Vec<i32>;
}

pub struct VecIO {
    input: Vec<i32>,
    input_pos: usize,
    output: Vec<i32>,
}
impl VecIO {
    pub fn new(input: Vec<i32>) -> VecIO {
        VecIO {
            input,
            input_pos: 0,
            output: vec![],
        }
    }
}
impl IO for VecIO {
    fn read(&mut self) -> Option<i32> {
        if self.input_pos >= self.input.len() {
            return None;
        }
        let result = Some(self.input[self.input_pos]);
        self.input_pos += 1;
        result
    }
    fn write(&mut self, val: i32) {
        self.output.push(val)
    }
    fn copy_output(&self) -> Vec<i32> {
        self.output.clone()
    }
}

#[derive(Clone, Copy)]
enum Mode {
    Position,
    Immediate,
}
impl Mode {
    fn new(code: i32) -> Mode {
        match code {
            0 => Mode::Position,
            1 => Mode::Immediate,
            _ => panic!("Unknown mode: {}", code),
        }
    }
    fn load(&self, arg: i32, memory: &[i32]) -> i32 {
        match self {
            Mode::Position => memory[usize::try_from(arg).unwrap()],
            Mode::Immediate => arg,
        }
    }
    fn write(&self, arg: i32, memory: &mut [i32], value: i32) {
        match self {
            Mode::Position => memory[usize::try_from(arg).unwrap()] = value,
            Mode::Immediate => panic!("Can't write to immediate"),
        }
    }
}

trait Modes {
    fn load(&self, params: &[i32], which_param: usize, memory: &[i32]) -> i32;
    fn write(&self, params: &[i32], which_param: usize, memory: &mut [i32], value: i32);
}
impl Modes for [Mode] {
    fn load(&self, params: &[i32], which_param: usize, memory: &[i32]) -> i32 {
        self[which_param].load(params[which_param], memory)
    }
    fn write(&self, params: &[i32], which_param: usize, memory: &mut [i32], value: i32) {
        self[which_param].write(params[which_param], memory, value)
    }
}

#[derive(Clone, Copy)]
enum Opcode {
    Add,
    Mul,
    In,
    Out,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}
impl Opcode {
    fn new(code: i32) -> Opcode {
        match code {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::In,
            4 => Opcode::Out,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::Halt,
            _ => panic!("Unexpected opcode: {}", code),
        }
    }
    fn params(&self) -> usize {
        match self {
            Opcode::Add => 3,
            Opcode::Mul => 3,
            Opcode::In => 1,
            Opcode::Out => 1,
            Opcode::JumpIfTrue => 2,
            Opcode::JumpIfFalse => 2,
            Opcode::LessThan => 3,
            Opcode::Equals => 3,
            Opcode::Halt => 0,
        }
    }
    fn execute(
        &self,
        params: &[i32],
        modes: &[Mode],
        program: &mut [i32],
        io: &mut dyn IO,
    ) -> OpcodeResult {
        assert_eq!(self.params(), params.len());
        assert_eq!(params.len(), modes.len());
        match self {
            Opcode::Add => {
                modes.write(
                    params,
                    2,
                    program,
                    modes.load(params, 0, program) + modes.load(params, 1, program),
                );
            }
            Opcode::Mul => {
                modes.write(
                    params,
                    2,
                    program,
                    modes.load(params, 0, program) * modes.load(params, 1, program),
                );
            }
            Opcode::In => {
                modes.write(params, 0, program, io.read().expect("Not enough input"));
            }
            Opcode::Out => {
                io.write(modes.load(params, 0, program));
            }
            Opcode::JumpIfTrue => {
                if modes.load(params, 0, program) != 0 {
                    return OpcodeResult::JumpTo(
                        modes.load(params, 1, program).try_into().unwrap(),
                    );
                }
            }
            Opcode::JumpIfFalse => {
                if modes.load(params, 0, program) == 0 {
                    return OpcodeResult::JumpTo(
                        modes.load(params, 1, program).try_into().unwrap(),
                    );
                }
            }
            Opcode::LessThan => {
                if modes.load(params, 0, program) < modes.load(params, 1, program) {
                    modes.write(params, 2, program, 1);
                } else {
                    modes.write(params, 2, program, 0);
                }
            }
            Opcode::Equals => {
                if modes.load(params, 0, program) == modes.load(params, 1, program) {
                    modes.write(params, 2, program, 1);
                } else {
                    modes.write(params, 2, program, 0);
                }
            }
            Opcode::Halt => {
                return OpcodeResult::Halt;
            }
        }
        OpcodeResult::Continue
    }
}

enum OpcodeResult {
    Continue,
    Halt,
    JumpTo(usize),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples_day2() {
        assert_eq!(
            run_intcode_input(vec![1, 0, 0, 0, 99], &[]).memory,
            vec![2, 0, 0, 0, 99]
        );
        assert_eq!(
            run_intcode_input(vec![2, 3, 0, 3, 99], &[]).memory,
            vec![2, 3, 0, 6, 99]
        );
        assert_eq!(
            run_intcode_input(vec![2, 4, 4, 5, 99, 0], &[]).memory,
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            run_intcode_input(vec![1, 1, 1, 4, 99, 5, 6, 0, 99], &[]).memory,
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    #[test]
    fn examples_day5_position_eq8() {
        let program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(run_intcode_input(program.clone(), &[7]).output, vec![0]);
        assert_eq!(run_intcode_input(program.clone(), &[8]).output, vec![1]);
        assert_eq!(run_intcode_input(program.clone(), &[9]).output, vec![0]);
    }

    #[test]
    fn examples_day5_position_lt8() {
        let program = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(run_intcode_input(program.clone(), &[7]).output, vec![1]);
        assert_eq!(run_intcode_input(program.clone(), &[8]).output, vec![0]);
        assert_eq!(run_intcode_input(program.clone(), &[9]).output, vec![0]);
    }

    #[test]
    fn examples_day5_imm_eq8() {
        let program = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(run_intcode_input(program.clone(), &[7]).output, vec![0]);
        assert_eq!(run_intcode_input(program.clone(), &[8]).output, vec![1]);
        assert_eq!(run_intcode_input(program.clone(), &[9]).output, vec![0]);
    }

    #[test]
    fn examples_day5_imm_lt8() {
        let program = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(run_intcode_input(program.clone(), &[7]).output, vec![1]);
        assert_eq!(run_intcode_input(program.clone(), &[8]).output, vec![0]);
        assert_eq!(run_intcode_input(program.clone(), &[9]).output, vec![0]);
    }

    #[test]
    fn examples_day5_jump_pos_nonzero() {
        let program = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        assert_eq!(run_intcode_input(program.clone(), &[-1]).output, vec![1]);
        assert_eq!(run_intcode_input(program.clone(), &[0]).output, vec![0]);
        assert_eq!(run_intcode_input(program.clone(), &[1]).output, vec![1]);
    }

    #[test]
    fn examples_day5_jump_imm_nonzero() {
        let program = vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1];
        assert_eq!(run_intcode_input(program.clone(), &[-1]).output, vec![1]);
        assert_eq!(run_intcode_input(program.clone(), &[0]).output, vec![0]);
        assert_eq!(run_intcode_input(program.clone(), &[1]).output, vec![1]);
    }

    #[test]
    fn examples_day5_longer_cmp_8() {
        let program = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        assert_eq!(run_intcode_input(program.clone(), &[7]).output, vec![999]);
        assert_eq!(run_intcode_input(program.clone(), &[8]).output, vec![1000]);
        assert_eq!(run_intcode_input(program.clone(), &[9]).output, vec![1001]);
    }
}
