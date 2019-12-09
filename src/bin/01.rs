use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> std::io::Result<()> {
    let mut file = File::open("input/01/input")?;
    let reader = BufReader::new(file);

    let fuel: u64 = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mass: f64 = line.parse().unwrap();
            calc_fuel(mass) as u64
        })
        .sum();

    println!("{}", fuel);

    Ok(())
}

fn calc_fuel(mass: f64) -> f64 {
    let fuel = ((mass / 3.).floor() - 2.);
    println!("needed: {}", fuel);
    if fuel <= 0.0 {
        return 0.;
    } else {
        return fuel + calc_fuel(fuel);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_fuel() {
        assert_eq!(calc_fuel(1969.), 966.);
    }
}
