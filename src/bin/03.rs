use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() {
    let mut file = File::open("input/03/input").unwrap();
    let reader = BufReader::new(file);

    let mut wire_paths: Vec<Vec<String>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .trim()
                .split(',')
                .map(|x| x.to_owned())
                .collect::<Vec<String>>()
        })
        .collect();

    //p1(&wire_paths[0], &wire_paths[1]);
    p2(&wire_paths[0], &wire_paths[1]);
}

fn p2<T: AsRef<str>>(wire_a: &[T], wire_b: &[T]) -> usize {
    let map_a = get_loc_set_extended(wire_a);
    let map_b = get_loc_set_extended(wire_b);

    let set_a: HashSet<_> = map_a.keys().cloned().collect();
    let set_b: HashSet<_> = map_b.keys().cloned().collect();

    let mut least = usize::max_value();
    for p in set_a.intersection(&set_b) {
        println!("INTERSECTION: {:?}", &p);
        let d1 = map_a.get(&p).unwrap();
        let d2 = map_b.get(&p).unwrap();
        println!("DISTANCES: {}, {}", d1, d2);
        if d1 + d2 < least {
            least = d1 + d2;
        }
    }
    println!("{}", least);
    least
}

fn get_loc_set_extended<T: AsRef<str>>(wire: &[T]) -> HashMap<(i64, i64), usize> {
    let mut hash_map = HashMap::new();
    let mut dist = 0;
    let mut pos: (i64, i64) = (0, 0);
    for path in wire {
        let dir = path.as_ref().chars().nth(0).unwrap();
        let amount: i64 = path.as_ref().get(1..).unwrap().parse().unwrap();

        for _ in 0..amount {
            match dir {
                'R' => pos.0 += 1,
                'L' => pos.0 -= 1,
                'U' => pos.1 += 1,
                'D' => pos.1 -= 1,
                _ => unreachable!(),
            }
            dist += 1;
            if hash_map.contains_key(&pos) {
                continue;
            } else {
                let overwrite = hash_map.insert(pos, dist);
            }
        }
    }
    return hash_map;
}

fn p1<T: AsRef<str>>(wire_a: &[T], wire_b: &[T]) -> i64 {
    let set_a = get_loc_set(wire_a);
    let set_b = get_loc_set(wire_b);

    println!("SET_A LENGTH: {}", set_a.len());
    println!("SET_B LENGTH: {}", set_b.len());

    let intersection = set_a.intersection(&set_b);
    let mut least = i64::max_value();
    println!("MAX: {}", least);
    for p in intersection {
        println!("INTERSECT: {:?}", p);
        let manhattan_dist = p.0.abs() + p.1.abs();
        if manhattan_dist < least && manhattan_dist != 0 {
            least = manhattan_dist;
        }
    }
    println!("{}", least);
    least
}

fn get_loc_set<T: AsRef<str>>(wire: &[T]) -> HashSet<(i64, i64)> {
    let mut set = HashSet::new();
    let mut pos: (i64, i64) = (0, 0);
    for path in wire {
        let dir = path.as_ref().chars().nth(0).unwrap();
        let amount: i64 = path.as_ref().get(1..).unwrap().parse().unwrap();

        println!("{} BY {} STARTING {:?}", &dir, &amount, &pos);
        for _ in 0..amount {
            match dir {
                'R' => pos.0 += 1,
                'L' => pos.0 -= 1,
                'U' => pos.1 += 1,
                'D' => pos.1 -= 1,
                _ => unreachable!(),
            }
            set.insert(pos);
        }
    }
    return set;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1_1() {
        let a = ["R8", "U5", "L5", "D3"];
        let b = ["U7", "R6", "D4", "L4"];
        assert_eq!(p1(&a, &b), 6)
    }

    #[test]
    #[ignore]
    fn test_1_2() {
        let a = ["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72"];
        let b = ["U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];
        assert_eq!(p1(&a, &b), 159)
    }

    #[test]
    fn test_2_1() {
        let a = ["R8", "U5", "L5", "D3"];
        let b = ["U7", "R6", "D4", "L4"];
        assert_eq!(p2(&a, &b), 30)
    }
}
