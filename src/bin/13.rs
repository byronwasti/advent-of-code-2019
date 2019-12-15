use advent_of_code_2019::cpu::IntCodeVM;
use advent_of_code_2019::logger;
use log::{debug, info};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::io::{stdin, stdout, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{thread, time};
use termion;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    logger::setup_logging();
    let mut file = File::open("input/13/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut prog: Vec<i64> = contents
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    println!("{}", p2(&mut prog));
}

fn input_handler(send: Sender<i32>) {
    let stdin = stdin();
    for c in stdin.keys() {
        let c = c.unwrap();
        debug!("KEY: {:?}", &c);
        match c {
            Key::Char('q') => break,
            Key::Char('h') => {
                send.send(-1);
            }
            Key::Char('l') => {
                send.send(1);
            }
            Key::Char('j') | Key::Char('k') => {
                send.send(0);
            }
            _ => {}
        }
    }

    send.send(-2);
}

fn p2(prog: &mut [i64]) -> i32 {
    prog[0] = 2;
    let (send, cpu_in) = channel();
    let (cpu_out, rec) = channel();
    let (in_signal_send, in_signal_rec) = channel();
    let mut cpu = IntCodeVM::new(prog, cpu_in, cpu_out, in_signal_send);
    thread::spawn(move || {
        cpu.run();
    });

    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::clear::All);

    let (input_send, input_rec) = channel();
    thread::spawn(move || {
        input_handler(input_send);
    });

    let mut buffer: Vec<_> = (0..=19)
        .map(|_| (0..=37).map(|_| ' ').collect::<Vec<_>>())
        .collect();

    let mut score = 0;
    let mut initialized = false;
    let mut paddle_pos = (0, 0);
    let mut ball_pos = (0, 0);
    loop {
        let send_val = match input_rec.try_recv() {
            Ok(x) => match x {
                -1 => -1,
                1 => 1,
                0 => 0,
                _ => break,
            },
            _ => {
                //debug!("PADDLE: ({}, {}), BALL: ({}, {})", paddle_pos.0, paddle_pos.1, ball_pos.0, ball_pos.1);
                match paddle_pos.0.cmp(&ball_pos.0) {
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                    Ordering::Less => 1,
                }
            }
        };

        if in_signal_rec.try_recv().is_ok() {
            debug!(
                "PADDLE: ({}, {}), BALL: ({}, {})",
                paddle_pos.0, paddle_pos.1, ball_pos.0, ball_pos.1
            );
            debug!("SENDING: {}", send_val);
            send.send(send_val);
        }

        let x = match rec.try_recv() {
            Ok(x) => x,
            _ => continue,
        };
        let y = rec.recv().unwrap();
        let tile_id = rec.recv().unwrap();

        if x == -1 && y == 0 {
            score = tile_id;
            initialized = true;
            write!(
                stdout,
                "{}{}Score: {}",
                termion::cursor::Goto(1, 22),
                termion::clear::CurrentLine,
                score
            );
        } else {
            let tile_char = match tile_id {
                0 => ' ',
                1 => '|',
                2 => 'X',
                3 => {
                    debug!("UPDATE PADDLE POS {}, {}", x, y);
                    paddle_pos = (x, y);
                    '-'
                }
                4 => {
                    debug!("UPDATE BALL POS {}, {}", x, y);
                    ball_pos = (x, y);
                    'o'
                }
                _ => panic!("Bad tile"),
            };

            buffer[y as usize][x as usize] = tile_char;
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(x as u16 + 1, y as u16 + 1),
                tile_char
            );
        }
        stdout.flush().unwrap();

        if initialized {
            thread::sleep(time::Duration::from_millis(1));
        }
    }
    0
}

/*
fn p1(prog: &[i64]) -> i32 {
    let (send, cpu_in) = channel();
    let (cpu_out, rec) = channel();

    let mut cpu = IntCodeVM::new(prog, cpu_in, cpu_out);
    thread::spawn(move || {
        cpu.run();
    });

    let mut block_tiles = 0;
    loop {
        let x = match rec.recv() {
            Ok(x) => x,
            _ => break,
        };
        let y = rec.recv().unwrap();
        let tile_id = rec.recv().unwrap();

        if tile_id == 2 {
            block_tiles += 1;
        }
    }
    block_tiles
}
*/
