use advent_of_code_2019::cpu;
use permutohedron::Heap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let mut file = File::open("input/07/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let prog: Vec<i32> = contents
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("{}", p2(Arc::new(prog)));
}

fn p2(prog: Arc<Vec<i32>>) -> i32 {
    let mut options = [5, 6, 7, 8, 9];
    let heap = Heap::new(&mut options);

    let mut max_val = 0;
    for permutation in heap {
        let mut fin = false;
        let mut io = 0;

        let (mut s01, r01) = channel();
        let (mut s12, r12) = channel();
        let (mut s23, r23) = channel();
        let (mut s34, r34) = channel();
        let (mut s40, r40) = channel();

        s40.send(permutation[0]);
        s01.send(permutation[1]);
        s12.send(permutation[2]);
        s23.send(permutation[3]);
        s34.send(permutation[4]);

        s40.send(0);

        let (mut sfinal, rfinal) = channel();
        let sfinal_opener = sfinal.clone();

        let p1 = prog.clone();
        let c1 = thread::spawn(move || {
            cpu::run_prog(&p1, r40, s01);
        });
        let p2 = prog.clone();
        let c2 = thread::spawn(move || {
            cpu::run_prog(&p2, r01, s12);
        });
        let p3 = prog.clone();
        let c3 = thread::spawn(move || {
            cpu::run_prog(&p3, r12, s23);
        });
        let p4 = prog.clone();
        let c4 = thread::spawn(move || {
            cpu::run_prog(&p4, r23, s34);
        });
        let p5 = prog.clone();
        let c5 = thread::spawn(move || {
            let out = cpu::run_prog(&p5, r34, s40);
            sfinal.send(out);
        });

        c1.join();
        c2.join();
        c3.join();
        c4.join();
        c5.join();

        let out = rfinal.recv().expect("Get Final Output");
        if out > max_val {
            max_val = out;
        }
    }

    max_val
}

/*
fn p1(prog: &[i32]) -> i32 {
    run_amplifier(prog, 0, 0, &HashSet::new())
}

fn run_amplifier(prog: &[i32], input: i32, depth: usize, used_phases: &HashSet<i32>) -> i32 {
    let mut max_val = 0;
    for i in 0..5 {
        if used_phases.contains(&i) {
            continue
        }

        let mut output = cpu::run_prog(&prog, &[i, input]).1[0];
        if depth != 4 {
            let mut used_phases_clone = used_phases.clone();
            used_phases_clone.insert(i);
            output = run_amplifier(prog, output, depth + 1, &used_phases_clone);
        }

        if output > max_val {
            max_val = output;
        }
    }
    max_val
}
*/

#[cfg(test)]
mod test {
    use super::*;

    /*
    #[test]
    #[ignore]
    fn test_1_1() {
        let prog = [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        assert_eq!(p1(&prog), 43210);
    }
    */

    #[test]
    #[ignore]
    fn test_2_1() {
        let prog = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(p2(Arc::new(prog)), 139629729);
    }

    #[test]
    fn test_2_2() {
        let prog = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(p2(Arc::new(prog)), 18216);
    }
}
