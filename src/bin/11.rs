use advent_of_code_2019::cpu::IntCodeVM;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

fn main() {
    let mut file = File::open("input/11/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let prog: Vec<i64> = contents
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    println!("{}", p2(&prog));
}

fn p2(prog: &[i64]) -> i32 {
    let (send, cpu_in) = channel();
    let (cpu_out, rec) = channel();
    let mut cpu = IntCodeVM::new(prog, cpu_in, cpu_out);
    thread::spawn(move || {
        cpu.run();
    });

    let mut panel_colors = HashMap::new();
    let mut pos = (0i64, 0i64);
    let mut direction = 0;
    let mut start = true;

    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    loop {
        if start {
            send.send(1);
            start = false;
        } else {
            match panel_colors.get(&pos) {
                Some(x) => {
                    send.send(*x);
                }
                None => {
                    send.send(0);
                }
            }
        }

        let new_color = match rec.recv() {
            Ok(x) => x,
            _ => break,
        };
        panel_colors.insert(pos, new_color);

        let turn = rec.recv().unwrap();
        if turn == 0 {
            direction -= 1;
            if direction < 0 {
                direction = 3;
            }
        } else {
            direction += 1;
            if direction > 3 {
                direction = 0;
            }
        }
        println!("{}", direction);

        match direction {
            0 => pos.1 += 1,
            1 => pos.0 += 1,
            2 => pos.1 -= 1,
            3 => pos.0 -= 1,
            _ => unreachable!(),
        }

        if pos.1 > max_y {
            max_y = pos.1;
        }
        if pos.1 < min_y {
            min_y = pos.1;
        }
        if pos.0 > max_x {
            max_x = pos.0;
        }
        if pos.0 < min_x {
            min_x = pos.0;
        }
    }

    println!("({}, {}) -> ({}, {})", min_x, min_y, max_x, max_y);

    let mut buffer: Vec<_> = (min_y..=max_y)
        .map(|_| (min_x..=max_x).map(|_| ' ').collect::<Vec<_>>())
        .collect();

    println!("{} [{}]", buffer.len(), buffer[0].len());

    for (key, val) in panel_colors.iter() {
        if *val == 0 {
            continue;
        }
        let x = key.0 - min_x;
        let y = key.1 - min_y;
        println!("({}, {}) => X", x, y);

        buffer[y as usize][x as usize] = 'X';
    }

    for y in buffer {
        println!("{:?}", y.into_iter().collect::<String>());
    }
    panel_colors.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {}
}
