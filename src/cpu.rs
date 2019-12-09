use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Read};
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct IntCodeVM {
    prog: Vec<i32>,
    input: Receiver<i32>,
    output: Sender<i32>,
}

impl IntCodeVM {
    pub fn new(prog: &[i32], input: Receiver<i32>, output: Sender<i32>) -> Self {
        let prog = prog.to_vec();
        Self {
            prog, input, output
        }
    }

    pub fn run(&mut self) -> i32 {
        let vals = &mut self.prog;
        let mut pc = 0;
        let mut output_val = 0;

        loop {
            let op = vals[pc];
            let (op, modes) = deconstruct_op(op);
            let op_char = match op {
                1 => {
                    let a = vals[pc + 1];
                    let b = vals[pc + 2];
                    let c = vals[pc + 3];
                    vals[c as usize] =
                        get_param(&vals, modes.get(0), a) + get_param(&vals, modes.get(1), b);
                    pc += 4;
                }
                2 => {
                    let a = vals[pc + 1];
                    let b = vals[pc + 2];
                    let c = vals[pc + 3];

                    vals[c as usize] =
                        get_param(&vals, modes.get(0), a) * get_param(&vals, modes.get(1), b);
                    pc += 4;
                }
                3 => {
                    let a = vals[pc + 1];
                    let val = self.input.recv().unwrap();
                    vals[a as usize] = val;
                    pc += 2;
                }
                4 => {
                    let a = vals[pc + 1];
                    let val = get_param(&vals, modes.get(0), a);
                    println!("OUT: {}", val);
                    output_val = val;
                    let _ = self.output.send(val);
                    pc += 2;
                }
                5 => {
                    let a = vals[pc + 1];
                    let b = vals[pc + 2];

                    if get_param(&vals, modes.get(0), a) != 0 {
                        pc = get_param(&vals, modes.get(1), b) as usize;
                    } else {
                        pc += 3;
                    }
                }
                6 => {
                    let a = vals[pc + 1];
                    let b = vals[pc + 2];

                    if get_param(&vals, modes.get(0), a) == 0 {
                        pc = get_param(&vals, modes.get(1), b) as usize;
                    } else {
                        pc += 3;
                    }
                }
                7 => {
                    let a = vals[pc + 1];
                    let b = vals[pc + 2];
                    let c = vals[pc + 3];

                    if get_param(&vals, modes.get(0), a, pc) < get_param(&vals, modes.get(1), b, pc) {
                        vals[c as usize] = 1;
                    } else {
                        vals[c as usize] = 0;
                    }

                    pc += 4;
                }
                8 => {
                    let a = vals[pc + 1];
                    let b = vals[pc + 2];
                    let c = vals[pc + 3];

                    if get_param(&vals, modes.get(0), a, pc) == get_param(&vals, modes.get(1), b, pc) {
                        vals[c as usize] = 1;
                    } else {
                        vals[c as usize] = 0;
                    }

                    pc += 4;
                }
                99 => {
                    break;
                }
                x => panic!(format!("Bad op: {} @ {}", x, pc)),
            };
        }
        output_val
    }
}

fn deconstruct_op(op: i32) -> (i32, Vec<i32>) {
    let mut op = op.to_string();

    let mode_set: Vec<_> = op
        .chars()
        .rev()
        .skip(2)
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect();

    let last_chr = op.pop();
    let second_last_chr = op.pop();

    let mut op = String::new();
    if let Some(c) = second_last_chr {
        op.push(c);
    }
    if let Some(c) = last_chr {
        op.push(c);
    }

    let op = op.parse::<i32>().unwrap();
    (op, mode_set)
}

fn get_param(vals: &[i32], mode: Option<&i32>, param: i32) -> i32 {
    match mode {
        Some(1) => param,
        _ => vals[param as usize],
    }
}
