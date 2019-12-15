use std::f64::consts::PI;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

use std::cmp::Ordering;
use std::collections::HashSet;
use std::convert::TryInto;

fn main() {
    let mut file = File::open("input/10/input").unwrap();
    let reader = BufReader::new(file);

    let mut asteroids = Vec::new();

    let mut max_y = 0;
    for (y, line) in reader.lines().enumerate() {
        max_y = y;
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.push((x as i32, y as i32));
            }
        }
    }

    println!("{:?}", p2(&asteroids, (20, 21)));
}

fn p2(asteroids: &[(i32, i32)], origin: (i32, i32)) -> i32 {
    let convert_to_polar = get_polar_func(origin);

    for a in asteroids {
        println!("{:?} -> {:?}", a, convert_to_polar(*a));
    }
    0
}

fn get_polar_func(origin: (i32, i32)) -> impl Fn((i32, i32)) -> (i32, i32) {
    move |a: (i32, i32)| {
        let a = (a.0 - origin.0, a.1 - origin.1);
        let r = a.0.pow(2) + a.1.pow(2);
        let theta = ((a.1 as f64).atan2(a.0 as f64) * 100.).round() as i32;
        (r, theta)
    }
}

fn p1(asteroids: &[(i32, i32)]) -> i32 {
    let mut max_found = 0;
    let mut max_loc = (0, 0);

    for main_asteroid in asteroids {
        let log_slopes = *main_asteroid == (5, 8);

        let mut slopes = HashSet::new();
        for comp_asteroid in asteroids {
            if comp_asteroid == main_asteroid {
                if log_slopes {
                    println!("compare skip: {:?}", comp_asteroid)
                }
                continue;
            }
            let slope = find_slope(*main_asteroid, *comp_asteroid);
            if log_slopes {
                println!("slope for {:?} is {:?}", comp_asteroid, slope)
            }
            if slopes.contains(&slope) {
                if log_slopes {
                    println!("slope contained")
                }
                continue;
            } else {
                if log_slopes {
                    println!("slope added")
                }
                slopes.insert(slope);
            }
        }

        if slopes.len() > max_found {
            max_found = slopes.len();
            max_loc = *main_asteroid;
        }
    }

    println!("MAX LOC: {:?}", max_loc);
    max_found.try_into().unwrap()
}

fn find_slope(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    let mx = (a.0 - b.0);
    let my = (a.1 - b.1);
    let gcd = gcd(mx.abs(), my.abs());
    (mx / gcd, my / gcd)
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_slopes() {
        assert_eq!(find_slope((5, 8), (8, 8)), (-1, 0));
    }

    fn convert_to_asteroids(s: &str) -> Vec<(i32, i32)> {
        let mut a = Vec::new();
        for (y, line) in s.split('\n').enumerate() {
            let line = line.trim();
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    a.push((x as i32, y as i32));
                }
            }
        }
        a
    }

    #[test]
    #[ignore]
    fn test_1_1() {
        let s = ".#..#
            .....
            #####
            ....#
            ...##";
        let a = convert_to_asteroids(&s);
        assert_eq!(p1(&a), 8);
    }

    #[test]
    #[ignore]
    fn test_1_2() {
        let s = "......#.#.
        #..#.#....
        ..#######.
        .#.#.###..
        .#..#.....
        ..#....#.#
        #..#....#.
        .##.#..###
        ##...#..#.
        .#....####";
        let a = convert_to_asteroids(&s);

        println!("{:?}", &a);
        assert_eq!(p1(&a), 33);
    }

    #[test]
    #[ignore]
    fn test_2_1() {
        let s = ".#....#####...#..
        ##...##.#####..##
        ##...#...#.#####.
        ..#.....#...###..
        ..#.#.....#....##";
        let a = convert_to_asteroids(&s);
        assert_eq!(p2(&a, (8, 3)), 5);
    }

    #[test]
    fn test_2_3() {
        let a = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        assert_eq!(p2(&a, (0, 0)), 10);
    }

    #[test]
    #[ignore]
    fn test_2_2() {
        let s = ".#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##";
        let a = convert_to_asteroids(&s);
        assert_eq!(p2(&a, (11, 13)), 802);
    }
}
