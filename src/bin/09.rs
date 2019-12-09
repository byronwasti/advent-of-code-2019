use advent_of_code_2019::cpu; 
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use advent_of_code_2019::cpu::IntCodeVM; 

fn main() {
    let mut file = File::open("input/09/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let prog: Vec<i32> = contents
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("{}", p1(&prog));
}

fn p1(prog: &[i32]) -> i32 {
    let (s, r) = channel();

    let mut cpu = IntCodeVM::new(prog, r, s);
    let t1 = thread::spawn(move || {
        cpu.run();
    });

    t1.join();
    0
}
