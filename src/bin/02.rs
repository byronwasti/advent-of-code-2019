use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/02/input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut vals: Vec<usize> = contents
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    println!("{}", p2(&vals));

    Ok(())
}

fn p1(vals: &[usize]) -> usize {
    let mut vals = vals.to_vec();
    vals[1] = 12;
    vals[2] = 2;
    let output = run_prog(&vals);

    println!("{}", output[0]);
    output[0]
}

fn p2(vals: &[usize]) -> usize {
    for i in 0..100 {
        for j in 0..100 {
            let mut vals = vals.to_vec();
            vals[1] = i;
            vals[2] = j;
            let output = run_prog(&vals);
            if output[0] == 19690720 {
                return 100 * i + j;
            }
        }
    }
    panic!("No solution")
}

fn run_prog(vals: &[usize]) -> Vec<usize> {
    let mut vals = vals.to_vec();
    let mut pc = 0;
    loop {
        let op = vals[pc];
        if op == 99 {
            break;
        }

        let a = vals[pc + 1];
        let b = vals[pc + 2];
        let c = vals[pc + 3];
        let op_char = match op {
            1 => {
                vals[c] = vals[a] + vals[b];
                '+'
            }
            2 => {
                vals[c] = vals[a] * vals[b];
                '*'
            }
            x => panic!(format!("Bad op: {} @ {}", x, pc)),
        };
        /*
        println!(
            "PC[{}]: vals[{}] = vals[{}]({}) {} vals[{}]({}) = {}",
            pc, c, a, vals[a], op_char, b, vals[b], vals[c]
        );
        */

        pc += 4;
    }

    vals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let prog = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        assert_eq!(run_prog(&prog)[0], 3500);
    }

    #[test]
    fn test_1_2() {
        let prog = [1, 0, 0, 0, 99];
        assert_eq!(run_prog(&prog), vec![2, 0, 0, 0, 99]);
    }

    #[test]
    fn test_1_3() {
        let prog = [2, 3, 0, 3, 99];
        assert_eq!(run_prog(&prog), vec![2, 3, 0, 6, 99]);
    }

    #[test]
    fn test_1_4() {
        let prog = [2, 4, 4, 5, 99, 0];
        assert_eq!(run_prog(&prog), vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn test_1_5() {
        let prog = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        assert_eq!(run_prog(&prog), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
