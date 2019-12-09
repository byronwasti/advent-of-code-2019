use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, Read};

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/05/input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut vals: Vec<i32> = contents
        .trim()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("{}", p1(&vals));

    Ok(())
}

fn p1(vals: &[i32]) -> i32 {
    let mut vals = vals.to_vec();
    let output = run_prog(&vals);

    println!("{}", output[0]);
    output[0]
}

fn run_prog(vals: &[i32]) -> Vec<i32> {
    let mut vals = vals.to_vec();
    let mut pc = 0;

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
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer);
                let val = buffer.trim().parse::<i32>().unwrap();
                vals[a as usize] = val;
                pc += 2;
            }
            4 => {
                let a = vals[pc + 1];
                println!("OUT: {}", get_param(&vals, modes.get(0), a));
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

                if get_param(&vals, modes.get(0), a) < get_param(&vals, modes.get(1), b) {
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

                if get_param(&vals, modes.get(0), a) == get_param(&vals, modes.get(1), b) {
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

    vals
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_get_modes() {
        assert_eq!(deconstruct_op(10100102), (2, vec![1, 0, 0, 1, 0, 1]))
    }

    #[test]
    fn test_1_1() {
        assert_eq!(run_prog(&[1101, 100, -1, 4, 0]), vec![1101, 100, -1, 4, 99])
    }

    #[test]
    #[ignore]
    fn test_1_2() {
        assert_eq!(run_prog(&[3, 0, 99]), vec![1, 0, 99])
    }
}
