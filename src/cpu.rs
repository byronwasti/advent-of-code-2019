use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Read};
use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct IntCodeVM {
    prog: Vec<i64>,
    input: Receiver<i64>,
    output: Sender<i64>,
    pc: usize,
    relative_base: isize,
}

impl IntCodeVM {
    pub fn new(prog: &[i64], input: Receiver<i64>, output: Sender<i64>) -> Self {
        let mut prog = prog.to_vec();
        let mut mem = vec![0; 1024]; 
        prog.append(&mut mem);
        Self {
            prog,
            input,
            output,
            pc: 0,
            relative_base: 0,
        }
    }

    fn get_value(&self, mode: Option<&i64>, param: i64) -> i64 {
        match mode {
            Some(1) => param,
            Some(2) => self.prog[(self.relative_base + param as isize) as usize],
            _ => self.prog[param as usize],
        }
    }

    fn set_value(&mut self, mode: Option<&i64>, param: i64, val: i64) {
        match mode {
            Some(2) => self.prog[(self.relative_base + param as isize) as usize] = val,
            _ => self.prog[param as usize] = val,
        }
    }

    pub fn run(&mut self) -> i64 {
        let mut output_val = 0;

        loop {
            let op = self.prog[self.pc];
            let (op, modes) = deconstruct_op(op);
            match op {
                1 => {
                    let p0 = self.prog[self.pc + 1];
                    let p1 = self.prog[self.pc + 2];
                    let p2 = self.prog[self.pc + 3];

                    let x = self.get_value(modes.get(0), p0);
                    let y = self.get_value(modes.get(1), p1);
                    let z = x + y;
                    self.set_value(modes.get(2), p2, z);

                    self.pc += 4;
                }
                2 => {
                    let p0 = self.prog[self.pc + 1];
                    let p1 = self.prog[self.pc + 2];
                    let p2 = self.prog[self.pc + 3];

                    let x = self.get_value(modes.get(0), p0);
                    let y = self.get_value(modes.get(1), p1);
                    let z = x * y;
                    self.set_value(modes.get(2), p2, z);
                    self.pc += 4;
                }
                3 => {
                    let p0 = self.prog[self.pc + 1];
                    let val = self.input.recv().unwrap();
                    self.set_value(modes.get(0), p0, val);
                    self.pc += 2;
                }
                4 => {
                    let p0 = self.prog[self.pc + 1];

                    let x = self.get_value(modes.get(0), p0);
                    let _ = self.output.send(x);
                    self.pc += 2;
                }
                5 => {
                    let p0 = self.prog[self.pc + 1];
                    let p1 = self.prog[self.pc + 2];

                    let x = self.get_value(modes.get(0), p0);
                    let y = self.get_value(modes.get(1), p1);

                    if x != 0 {
                        self.pc = y as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                6 => {
                    let p0 = self.prog[self.pc + 1];
                    let p1 = self.prog[self.pc + 2];

                    let x = self.get_value(modes.get(0), p0);
                    let y = self.get_value(modes.get(1), p1);

                    if x == 0 {
                        self.pc = y as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                7 => {
                    let p0 = self.prog[self.pc + 1];
                    let p1 = self.prog[self.pc + 2];
                    let p2 = self.prog[self.pc + 3];

                    let x = self.get_value(modes.get(0), p0);
                    let y = self.get_value(modes.get(1), p1);

                    if x < y {
                        self.set_value(modes.get(2), p2, 1);
                    } else {
                        self.set_value(modes.get(2), p2, 0);
                    }

                    self.pc += 4;
                }
                8 => {
                    let p0 = self.prog[self.pc + 1];
                    let p1 = self.prog[self.pc + 2];
                    let p2 = self.prog[self.pc + 3];

                    let x = self.get_value(modes.get(0), p0);
                    let y = self.get_value(modes.get(1), p1);

                    if x == y {
                        self.set_value(modes.get(2), p2, 1);
                    } else {
                        self.set_value(modes.get(2), p2, 0);
                    }

                    self.pc += 4;
                }
                9 => {
                    let p0 = self.prog[self.pc + 1];

                    let x = self.get_value(modes.get(0), p0);
                    self.relative_base += x as isize;
                    self.pc += 2;
                }
                99 => {
                    break;
                }
                x => panic!(format!("Bad op: {} @ {}", x, self.pc)),
            }
        }
        output_val
    }
}

fn deconstruct_op(op: i64) -> (i64, Vec<i64>) {
    let mut op = op.to_string();

    let mode_set: Vec<_> = op
        .chars()
        .rev()
        .skip(2)
        .map(|x| x.to_digit(10).unwrap() as i64)
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

    let op = op.parse::<i64>().unwrap();
    (op, mode_set)
}
