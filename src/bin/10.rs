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

    asteroids = asteroids
        .iter()
        .map(|(x, y)| (*x, max_y as i32 - y))
        .collect::<Vec<_>>();

    println!("{:?}", p2(&asteroids, (20, 21)));
}

fn p2_(asteroids: &[(i32, i32)], origin: (i32, i32)) -> i32 {
    0
}

fn p2(asteroids: &[(i32, i32)], origin: (i32, i32)) -> i32 {
    let asteroids_origin: Vec<_> = asteroids
        .iter()
        .map(|(x, y)| (x - origin.0, origin.1 - y))
        .filter(|c| *c != (0, 0))
        .collect();

    let polar: Vec<_> = asteroids_origin
        .iter()
        //.map(|coord| convert_to_polar(*coord, (0, 0)))
        .map(|coord| convert_to_polar((0, 0), *coord))
        .collect();

    let mut polar_cart: Vec<_> = polar.iter().zip(asteroids).collect();
    polar_cart.sort_unstable_by(|(p1, _), (p2, _)| {
        let initial = p1.1.partial_cmp(&p2.1).unwrap();
        if initial == Ordering::Equal {
            p1.0.partial_cmp(&p2.0).unwrap()
        } else {
            initial
        }
    });

    /*
    for (p, c) in polar_cart {
        println!("{:?} -> {:?}", c, p);
    }
    */

    let mut eliminated = HashSet::new();
    let goal_len = polar_cart.len();
    let mut the_one = (0, 0);

    while eliminated.len() != goal_len {
        let mut prev: Option<f64> = None;
        for (p, c) in &polar_cart {
            if eliminated.contains(&c) {
                continue;
            }
            if let Some(prev) = prev {
                if p.1 == prev {
                    continue;
                }
            }

            eliminated.insert(c);
            prev = Some(p.1);
            println!("ELIMINATED: {:?} ({:?})", c, p);
            //println!("ELIMINATED: {:?}", (c.0 + origin.0, c.1 + origin.1));
            if eliminated.len() == 200 {
                the_one = **c;
            }
        }
    }

    the_one.0 * 100 + the_one.1
}

fn p2__(asteroids: &[(i32, i32)], origin: (i32, i32)) -> i32 {
    // MAX LOC: (20, 21)

    let mut polar_coords: Vec<((i32, f64), (i32, i32))> = Vec::new();
    for asteroid in asteroids {
        if *asteroid == origin {
            continue;
        }

        let polar = convert_to_polar(origin, *asteroid);
        let val = (polar, *asteroid);
        polar_coords.push(val);
    }

    polar_coords.sort_unstable_by(|(p1, _), (p2, _)| {
        let initial = p1.1.partial_cmp(&p2.1).unwrap();
        if initial == Ordering::Equal {
            p1.0.partial_cmp(&p2.0).unwrap()
        } else {
            initial
        }
    });

    for (p, c) in polar_coords {
        println!(
            "{:?}  ->  {:?} ({:?})",
            p,
            c,
            (c.0 - origin.0, c.1 - origin.1)
        );
    }
    2
}

fn convert_to_polar(origin: (i32, i32), coord: (i32, i32)) -> (i32, f64) {
    let x = coord.0 - origin.0;
    let y = coord.1 - origin.1;

    let r_squared = x.pow(2) + y.pow(2);
    let theta = (y as f64).atan2(x as f64) + PI;
    let theta = theta % (2. * PI);
    (r_squared, theta)
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
    fn test_convert_to_polar_1() {
        assert_eq!(convert_to_polar((0, 0), (1, 0)), (1, 0.));
        assert_eq!(convert_to_polar((0, 0), (-1, 0)), (1, 3. * PI / 2.));
        assert_eq!(convert_to_polar((0, 0), (0, 1)), (1, 0.));
        assert_eq!(convert_to_polar((0, 0), (0, -1)), (1, PI));
    }

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
