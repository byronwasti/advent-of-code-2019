use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

use std::collections::HashMap;

fn main() {
    let mut file = File::open("input/06/input").unwrap();
    let reader = BufReader::new(file);

    let mut orbit_pairs = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let line_split: Vec<_> = line.split(')').collect();
        let a = line_split[0].to_string();
        let b = line_split[1].to_string();
        orbit_pairs.push((a, b));
    }

    println!("{}", p2(&orbit_pairs));
}

type RevOrbitMap<'a> = HashMap<&'a String, Vec<&'a String>>;
type OrbitTree<'a> = HashMap<String, (usize, String)>;

fn p2(orbit_pairs: &[(String, String)]) -> usize {
    let mut rev_orbit_map: RevOrbitMap = HashMap::new();
    for orbit_pair in orbit_pairs {
        if rev_orbit_map.contains_key(&orbit_pair.0) {
            let v = rev_orbit_map.get_mut(&orbit_pair.0).unwrap();
            v.push(&orbit_pair.1);
        } else {
            let v = vec![&orbit_pair.1];
            rev_orbit_map.insert(&orbit_pair.0, v);
        }
    }
    let mut orbit_tree = OrbitTree::new();
    for planet in rev_orbit_map.get(&"COM".to_owned()).unwrap() {
        build_orbit_tree(&rev_orbit_map, &mut orbit_tree, &"COM".to_owned(), &planet, 1);
    }

    let mut pyou = orbit_tree.get(&"YOU".to_owned()).unwrap();
    let mut psan = orbit_tree.get(&"SAN".to_owned()).unwrap();
    let mut orbit_hops = 0;
    loop {
        if pyou.0 == psan.0 && pyou.1 == psan.1 {
            println!("ORBIT HOPS: {}", orbit_hops);
            return orbit_hops;
        } else if pyou.0 == psan.0 {
            pyou = orbit_tree.get(&pyou.1).unwrap();
            psan = orbit_tree.get(&psan.1).unwrap();
            orbit_hops += 2;
        } else if pyou.0 > psan.0 {
            pyou = orbit_tree.get(&pyou.1).unwrap();
            orbit_hops += 1;
        } else {
            psan = orbit_tree.get(&psan.1).unwrap();
            orbit_hops += 1;
        }
    }
}

fn build_orbit_tree<'a>(
    rev_orbit_map: &RevOrbitMap,
    orbit_tree: &mut OrbitTree,
    parent: &String,
    planet: &String,
    depth: usize,
) {
    if rev_orbit_map.contains_key(planet) {
        for next in rev_orbit_map.get(planet).unwrap() {
            build_orbit_tree(rev_orbit_map, orbit_tree, planet, next, depth + 1);
        }
    }
    orbit_tree.insert(planet.to_owned(), (depth, parent.to_owned()));
}

fn p1(orbit_pairs: &[(String, String)]) -> usize {
    let mut rev_orbit_map: RevOrbitMap = HashMap::new();
    for orbit_pair in orbit_pairs {
        if rev_orbit_map.contains_key(&orbit_pair.0) {
            let v = rev_orbit_map.get_mut(&orbit_pair.0).unwrap();
            v.push(&orbit_pair.1);
        } else {
            let v = vec![&orbit_pair.1];
            rev_orbit_map.insert(&orbit_pair.0, v);
        }
    }

    count_orbits(&rev_orbit_map, &"COM".to_string(), 0)
}

fn count_orbits(rev_orbit_map: &RevOrbitMap, start: &String, depth: usize) -> usize {
    if !rev_orbit_map.contains_key(start) {
        return depth;
    }

    let mut sum = 0;
    for orbit in rev_orbit_map.get(start).unwrap() {
        sum += count_orbits(rev_orbit_map, orbit, depth + 1);
    }
    return sum + depth;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1_1() {
        let orbit_pairs = [
            ("COM".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
            ("C".to_string(), "D".to_string()),
            ("D".to_string(), "E".to_string()),
            ("E".to_string(), "F".to_string()),
            ("B".to_string(), "G".to_string()),
            ("G".to_string(), "H".to_string()),
            ("D".to_string(), "I".to_string()),
            ("E".to_string(), "J".to_string()),
            ("J".to_string(), "K".to_string()),
            ("K".to_string(), "L".to_string()),
        ];
        assert_eq!(p1(&orbit_pairs), 42);
    }

    #[test]
    fn test_2_1() {
        let orbit_pairs = [
            ("COM".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
            ("C".to_string(), "D".to_string()),
            ("D".to_string(), "E".to_string()),
            ("E".to_string(), "F".to_string()),
            ("B".to_string(), "G".to_string()),
            ("G".to_string(), "H".to_string()),
            ("D".to_string(), "I".to_string()),
            ("E".to_string(), "J".to_string()),
            ("J".to_string(), "K".to_string()),
            ("K".to_string(), "L".to_string()),
            ("K".to_string(), "YOU".to_string()),
            ("I".to_string(), "SAN".to_string()),
        ];
        assert_eq!(p2(&orbit_pairs), 4);
    }
}
