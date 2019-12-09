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

    let prog: Vec<i64> = contents
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    println!("{}", p1(&prog));
}

fn p1(prog: &[i64]) -> i64 {
    let (send, cpu_in) = channel();
    let (cpu_out, rec) = channel();

    send.send(2);
    let mut cpu = IntCodeVM::new(prog, cpu_in, cpu_out);
    let val = cpu.run();
    val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let prog = [109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    }
}
