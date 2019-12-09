use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};


fn main() {
    let mut file = File::open("input/08/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let vals = string_to_nums(&contents);

    //println!("{}", p1(&vals, (25, 6)));
    p2(&vals, (25, 6));
}

fn string_to_nums(s: &str) -> Vec<i32> {
    s.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

fn p2(vals: &[i32], size: (usize, usize)) {
    let mut buffer: Vec<_> = (0..(size.0 * size.1)).map(|_| 2).collect();
    for chunk in vals.chunks(size.0 * size.1) {
        for (idx, pixel) in chunk.iter().enumerate() {
            if buffer[idx] != 2 {
                continue
            }

            match pixel {
                0 => buffer[idx] = 0,
                1 => buffer[idx] = 1,
                2 => {},
                _ => panic!("Bad pixel value"),
            }
        }
    }

    for chunk in buffer.chunks(size.0) {
        println!("{:?}", chunk);
    }
}

fn p1(vals: &[i32], size: (usize, usize)) -> i32 {
    let mut metrics = Vec::new();
    for chunk in vals.chunks(size.0 * size.1) {
        let mut zeros = 0;
        let mut ones = 0;
        let mut twos = 0;
        for i in chunk {
            match i {
                0 => zeros += 1,
                1 => ones += 1,
                2 => twos += 1,
                _ => {}
            }
        }
        metrics.push((zeros, ones * twos));
    }

    metrics.iter().min_by_key(|x| x.0).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_1() {
        let vals = string_to_nums(&"123456789012");
        assert_eq!(p1(&vals, (3, 2)), 1)
    }
}
