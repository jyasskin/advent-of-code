use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(PartialEq, Eq, Debug)]
pub struct IntcodeResult {
    pub memory: Vec<i64>,
    pub output: Vec<i64>,
}

pub fn run_intcode(mut program: Vec<i64>, state: &mut dyn State) -> IntcodeResult {
    let mut position = 0;
    loop {
        let opcode = Opcode::new(program[position] % 100);
        let mut allmodes = program[position] / 100;
        let mut modes: Vec<i64> = vec![];
        while allmodes != 0 {
            modes.push(allmodes % 10);
            allmodes /= 10;
        }
        modes.resize(opcode.params(), 0);
        let modes: Vec<Mode> = modes.into_iter().map(Mode::new).collect();
        let params = Box::from(&program[position + 1..position + 1 + opcode.params()]);
        match opcode.execute(&params, &modes, &mut program, state) {
            OpcodeResult::Continue => position += opcode.params() + 1,
            OpcodeResult::JumpTo(target) => position = target,
            OpcodeResult::Halt => {
                return IntcodeResult {
                    memory: program,
                    output: state.copy_output(),
                }
            }
        }
    }
}

pub fn run_intcode_input(program: Vec<i64>, input: &[i64]) -> IntcodeResult {
    run_intcode(program, &mut VecState::new(input.into()))
}

pub trait State {
    fn input(&mut self) -> Option<i64>;
    fn output(&mut self, val: i64);
    fn copy_output(&self) -> Vec<i64>;
    fn adjust_relative_base(&mut self, adjust: i64);
    fn relative_base(&self) -> i64;
}

pub struct VecState {
    input: Vec<i64>,
    input_pos: usize,
    output: Vec<i64>,
    relative_base: i64,
}
impl VecState {
    pub fn new(input: Vec<i64>) -> VecState {
        VecState {
            input,
            input_pos: 0,
            output: vec![],
            relative_base: 0,
        }
    }
}
impl State for VecState {
    fn input(&mut self) -> Option<i64> {
        if self.input_pos >= self.input.len() {
            return None;
        }
        let result = Some(self.input[self.input_pos]);
        self.input_pos += 1;
        result
    }
    fn output(&mut self, val: i64) {
        self.output.push(val)
    }
    fn copy_output(&self) -> Vec<i64> {
        self.output.clone()
    }
    fn adjust_relative_base(&mut self, adjust: i64) {
        self.relative_base += adjust;
    }
    fn relative_base(&self) -> i64 {
        self.relative_base
    }
}

fn extend_read(v: &Vec<i64>, index: usize) -> i64 {
    if index >= v.len() {
        0
    } else {
        v[index]
    }
}

fn extend_write(v: &mut Vec<i64>, index: usize, val: i64) {
    if index >= v.len() {
        v.resize(index + 1, 0);
    }
    v[index] = val;
}

#[derive(Clone, Copy)]
enum Mode {
    Position,
    Immediate,
    Relative,
}
impl Mode {
    fn new(code: i64) -> Mode {
        match code {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("Unknown mode: {}", code),
        }
    }
    fn load(&self, arg: i64, relative_base: i64, memory: &Vec<i64>) -> i64 {
        match self {
            Mode::Position => extend_read(memory, usize::try_from(arg).unwrap()),
            Mode::Immediate => arg,
            Mode::Relative => extend_read(memory, usize::try_from(arg + relative_base).unwrap()),
        }
    }
    fn write(&self, arg: i64, relative_base: i64, memory: &mut Vec<i64>, value: i64) {
        match self {
            Mode::Position => extend_write(memory, usize::try_from(arg).unwrap(), value),
            Mode::Immediate => panic!("Can't write to immediate"),
            Mode::Relative => {
                extend_write(memory, usize::try_from(arg + relative_base).unwrap(), value)
            }
        }
    }
}

trait Modes {
    fn load(
        &self,
        params: &[i64],
        which_param: usize,
        relative_base: i64,
        memory: &Vec<i64>,
    ) -> i64;
    fn write(
        &self,
        params: &[i64],
        which_param: usize,
        relative_base: i64,
        memory: &mut Vec<i64>,
        value: i64,
    );
}
impl Modes for [Mode] {
    fn load(
        &self,
        params: &[i64],
        which_param: usize,
        relative_base: i64,
        memory: &Vec<i64>,
    ) -> i64 {
        self[which_param].load(params[which_param], relative_base, memory)
    }
    fn write(
        &self,
        params: &[i64],
        which_param: usize,
        relative_base: i64,
        memory: &mut Vec<i64>,
        value: i64,
    ) {
        self[which_param].write(params[which_param], relative_base, memory, value)
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
    AdjustRelativeBase,
    Halt,
}
impl Opcode {
    fn new(code: i64) -> Opcode {
        match code {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::In,
            4 => Opcode::Out,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            9 => Opcode::AdjustRelativeBase,
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
            Opcode::AdjustRelativeBase => 1,
            Opcode::Halt => 0,
        }
    }
    fn execute(
        &self,
        params: &[i64],
        modes: &[Mode],
        program: &mut Vec<i64>,
        state: &mut dyn State,
    ) -> OpcodeResult {
        assert_eq!(self.params(), params.len());
        assert_eq!(params.len(), modes.len());
        match self {
            Opcode::Add => {
                modes.write(
                    params,
                    2,
                    state.relative_base(),
                    program,
                    modes.load(params, 0, state.relative_base(), program)
                        + modes.load(params, 1, state.relative_base(), program),
                );
            }
            Opcode::Mul => {
                modes.write(
                    params,
                    2,
                    state.relative_base(),
                    program,
                    modes.load(params, 0, state.relative_base(), program)
                        * modes.load(params, 1, state.relative_base(), program),
                );
            }
            Opcode::In => {
                modes.write(
                    params,
                    0,
                    state.relative_base(),
                    program,
                    state.input().expect("Not enough input"),
                );
            }
            Opcode::Out => {
                state.output(modes.load(params, 0, state.relative_base(), program));
            }
            Opcode::JumpIfTrue => {
                if modes.load(params, 0, state.relative_base(), program) != 0 {
                    return OpcodeResult::JumpTo(
                        modes
                            .load(params, 1, state.relative_base(), program)
                            .try_into()
                            .unwrap(),
                    );
                }
            }
            Opcode::JumpIfFalse => {
                if modes.load(params, 0, state.relative_base(), program) == 0 {
                    return OpcodeResult::JumpTo(
                        modes
                            .load(params, 1, state.relative_base(), program)
                            .try_into()
                            .unwrap(),
                    );
                }
            }
            Opcode::LessThan => {
                if modes.load(params, 0, state.relative_base(), program)
                    < modes.load(params, 1, state.relative_base(), program)
                {
                    modes.write(params, 2, state.relative_base(), program, 1);
                } else {
                    modes.write(params, 2, state.relative_base(), program, 0);
                }
            }
            Opcode::Equals => {
                if modes.load(params, 0, state.relative_base(), program)
                    == modes.load(params, 1, state.relative_base(), program)
                {
                    modes.write(params, 2, state.relative_base(), program, 1);
                } else {
                    modes.write(params, 2, state.relative_base(), program, 0);
                }
            }
            Opcode::AdjustRelativeBase => {
                state.adjust_relative_base(modes.load(params, 0, state.relative_base(), program));
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

    #[test]
    fn examples_day9() {
        assert_eq!(
            run_intcode_input(
                vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
                &[]
            )
            .output,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );

        assert_eq!(
            run_intcode_input(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0], &[]).output,
            vec![1219070632396864]
        );

        assert_eq!(
            run_intcode_input(vec![104, 1125899906842624, 99], &[]).output,
            vec![1125899906842624]
        );
    }
}
