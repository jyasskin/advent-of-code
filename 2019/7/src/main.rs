use intcode::*;
use std::io;
use std::io::BufRead;
use std::sync::mpsc;
use std::thread;

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .expect("Couldn't read input");
    let (signal, phases) = find_max_signal(parse_intcode(&lines[0]));
    println!("Part 1: Max signal: {}; Max phases: {:?}", signal, phases);
    let (signal, phases) = find_max_feedback_signal(parse_intcode(&lines[0]));
    println!("Part 2: Max signal: {}; Max phases: {:?}", signal, phases);
}

fn find_max_signal(code: Vec<i32>) -> (i32, [i32; 5]) {
    let mut max_signal: i32 = i32::min_value();
    let mut max_phases: [i32; 5] = [0, 0, 0, 0, 0];
    for aphase in 0..5 {
        for bphase in 0..5 {
            if aphase == bphase {
                continue;
            }
            for cphase in 0..5 {
                if cphase == bphase || cphase == aphase {
                    continue;
                }
                for dphase in 0..5 {
                    if dphase == cphase || dphase == bphase || dphase == aphase {
                        continue;
                    }
                    for ephase in 0..5 {
                        if ephase == dphase
                            || ephase == cphase
                            || ephase == bphase
                            || ephase == aphase
                        {
                            continue;
                        }
                        let IntcodeResult {
                            output: a_output, ..
                        } = run_intcode_input(code.clone(), &[aphase, 0]);
                        let IntcodeResult {
                            output: b_output, ..
                        } = run_intcode_input(code.clone(), &[bphase, a_output[0]]);
                        let IntcodeResult {
                            output: c_output, ..
                        } = run_intcode_input(code.clone(), &[cphase, b_output[0]]);
                        let IntcodeResult {
                            output: d_output, ..
                        } = run_intcode_input(code.clone(), &[dphase, c_output[0]]);
                        let IntcodeResult {
                            output: e_output, ..
                        } = run_intcode_input(code.clone(), &[ephase, d_output[0]]);

                        if e_output[0] > max_signal {
                            max_signal = e_output[0];
                            max_phases = [aphase, bphase, cphase, dphase, ephase];
                        }
                    }
                }
            }
        }
    }
    (max_signal, max_phases)
}

struct ChannelIO {
    input: mpsc::Receiver<i32>,
    output: mpsc::Sender<i32>,
    excess_output: Vec<i32>,
}
impl IO for ChannelIO {
    fn read(&mut self) -> Option<i32> {
        let result = self.input.recv().ok();
        result
    }
    fn write(&mut self, val: i32) {
        match self.output.send(val) {
            Result::Err(mpsc::SendError(err)) => self.excess_output.push(err),
            _ => {}
        }
    }
    fn copy_output(&self) -> Vec<i32> {
        self.excess_output.clone()
    }
}

fn find_max_feedback_signal(code: Vec<i32>) -> (i32, [i32; 5]) {
    let mut max_signal: i32 = i32::min_value();
    let mut max_phases: [i32; 5] = [0, 0, 0, 0, 0];
    for aphase in 5..10 {
        for bphase in 5..10 {
            if aphase == bphase {
                continue;
            }
            for cphase in 5..10 {
                if cphase == bphase || cphase == aphase {
                    continue;
                }
                for dphase in 5..10 {
                    if dphase == cphase || dphase == bphase || dphase == aphase {
                        continue;
                    }
                    for ephase in 5..10 {
                        if ephase == dphase
                            || ephase == cphase
                            || ephase == bphase
                            || ephase == aphase
                        {
                            continue;
                        }
                        let (ab_sender, ab_receiver) = mpsc::channel();
                        let (bc_sender, bc_receiver) = mpsc::channel();
                        let (cd_sender, cd_receiver) = mpsc::channel();
                        let (de_sender, de_receiver) = mpsc::channel();
                        let (ea_sender, ea_receiver) = mpsc::channel();
                        ea_sender.send(aphase).expect("Writing A's phase.");
                        ab_sender.send(bphase).expect("Writing B's phase.");
                        bc_sender.send(cphase).expect("Writing C's phase.");
                        cd_sender.send(dphase).expect("Writing D's phase.");
                        de_sender.send(ephase).expect("Writing E's phase.");
                        ea_sender.send(0).expect("Writing A's initial input.");

                        let mut a_io = ChannelIO {
                            input: ea_receiver,
                            output: ab_sender,
                            excess_output: vec![],
                        };
                        let mut b_io = ChannelIO {
                            input: ab_receiver,
                            output: bc_sender,
                            excess_output: vec![],
                        };
                        let mut c_io = ChannelIO {
                            input: bc_receiver,
                            output: cd_sender,
                            excess_output: vec![],
                        };
                        let mut d_io = ChannelIO {
                            input: cd_receiver,
                            output: de_sender,
                            excess_output: vec![],
                        };
                        let mut e_io = ChannelIO {
                            input: de_receiver,
                            output: ea_sender,
                            excess_output: vec![],
                        };
                        let a_code = code.clone();
                        let b_code = code.clone();
                        let c_code = code.clone();
                        let d_code = code.clone();
                        let e_code = code.clone();
                        let handles = vec![
                            thread::Builder::new()
                                .name("A".into())
                                .spawn(move || run_intcode(a_code, &mut a_io))
                                .unwrap(),
                            thread::Builder::new()
                                .name("B".into())
                                .spawn(move || run_intcode(b_code, &mut b_io))
                                .unwrap(),
                            thread::Builder::new()
                                .name("C".into())
                                .spawn(move || run_intcode(c_code, &mut c_io))
                                .unwrap(),
                            thread::Builder::new()
                                .name("D".into())
                                .spawn(move || run_intcode(d_code, &mut d_io))
                                .unwrap(),
                        ];
                        let e_handle = thread::Builder::new()
                            .name("E".into())
                            .spawn(move || run_intcode(e_code, &mut e_io))
                            .unwrap();

                        for handle in handles {
                            handle.join().expect("Join of A-D failed");
                        }

                        let e_signal = *e_handle
                            .join()
                            .expect("Join of E failed")
                            .output
                            .last()
                            .expect("Expected output from E");
                        if e_signal > max_signal {
                            max_signal = e_signal;
                            max_phases = [aphase, bphase, cphase, dphase, ephase];
                        }
                    }
                }
            }
        }
    }
    (max_signal, max_phases)
}

fn parse_intcode(code: &str) -> Vec<i32> {
    code.split(',')
        .map(|s| s.parse().expect("Should be an integer"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!(
            find_max_signal(parse_intcode(
                "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
            )),
            (43210, [4, 3, 2, 1, 0])
        );
        assert_eq!(
            find_max_signal(parse_intcode(
                "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            )),
            (54321, [0, 1, 2, 3, 4])
        );
        assert_eq!(
            find_max_signal(parse_intcode(
                "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            )),
            (65210, [1,0,4,3,2])
        );
    }

    #[test]
    fn examples_part2() {
        assert_eq!(
            find_max_feedback_signal(parse_intcode(
                "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
            )),
            (139629729, [9,8,7,6,5])
        );
        assert_eq!(
            find_max_feedback_signal(parse_intcode(
                "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
            )),
            (18216, [9,7,8,5,6])
        );
    }
}
