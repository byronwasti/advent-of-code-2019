use std::cmp::Ordering;
use std::collections::HashSet;

fn main() {
    let m1 = Moon::new((-10, -10, -13), (0, 0, 0));
    let m2 = Moon::new((5, 5, -9), (0, 0, 0));
    let m3 = Moon::new((3, 8, -16), (0, 0, 0));
    let m4 = Moon::new((1, 3, -3), (0, 0, 0));

    let mut moons = [m1, m2, m3, m4];
    println!("{}", p2(&mut moons));
}

fn p2(moons: &mut [Moon]) -> i64 {
    let initial_state = moons.iter().map(|m| m.clone()).collect::<Vec<_>>();

    let pairs: Vec<_> = (0..4)
        .flat_map(|i| {
            (i + 1..4)
                .map(|x| (i as usize, x as usize))
                .collect::<Vec<_>>()
        })
        .collect();

    println!("{:?}", &pairs);

    let mut i = 0;
    loop {
        i += 1;
        for (a, b) in &pairs {
            let (s0, s1) = moons.split_at_mut(*b);
            apply_gravity(&mut s0[*a], &mut s1[0]);
        }

        for m in moons.iter_mut() {
            m.update_pos();
        }

        let mut back_to_start = true;
        for (m, m0) in moons.iter().zip(&initial_state) {
            if m.get_state_z() != m0.get_state_z() {
                back_to_start = false;
            }
        }
        if back_to_start {
            break;
        }
    }

    i
}

fn p1(moons: &mut [Moon]) -> i64 {
    let pairs: Vec<_> = (0..4)
        .flat_map(|i| {
            (i + 1..4)
                .map(|x| (i as usize, x as usize))
                .collect::<Vec<_>>()
        })
        .collect();

    println!("{:?}", &pairs);

    for _ in 0..1000 {
        for (a, b) in &pairs {
            let (s0, s1) = moons.split_at_mut(*b);
            apply_gravity(&mut s0[*a], &mut s1[0]);
        }

        for m in moons.iter_mut() {
            m.update_pos();
        }
    }

    moons.iter().map(|m| m.get_energy()).sum()
}

fn apply_gravity(m1: &mut Moon, m2: &mut Moon) {
    let pos1 = m1.get_pos();
    let pos2 = m2.get_pos();

    let mut delta_m1 = (0, 0, 0);
    let mut delta_m2 = (0, 0, 0);

    use Ordering::*;
    match pos1.0.partial_cmp(&pos2.0).unwrap() {
        Less => {
            delta_m1.0 += 1;
            delta_m2.0 -= 1;
        }
        Greater => {
            delta_m1.0 -= 1;
            delta_m2.0 += 1;
        }
        _ => {}
    }

    match pos1.1.partial_cmp(&pos2.1).unwrap() {
        Less => {
            delta_m1.1 += 1;
            delta_m2.1 -= 1;
        }
        Greater => {
            delta_m1.1 -= 1;
            delta_m2.1 += 1;
        }
        _ => {}
    }

    match pos1.2.partial_cmp(&pos2.2).unwrap() {
        Less => {
            delta_m1.2 += 1;
            delta_m2.2 -= 1;
        }
        Greater => {
            delta_m1.2 -= 1;
            delta_m2.2 += 1;
        }
        _ => {}
    }

    m1.apply_delta(delta_m1);
    m2.apply_delta(delta_m2);
}

#[derive(PartialEq, Copy, Clone)]
struct Moon {
    x: i64,
    y: i64,
    z: i64,
    dx: i64,
    dy: i64,
    dz: i64,
}

impl Moon {
    pub fn new(pos: (i64, i64, i64), vel: (i64, i64, i64)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
            z: pos.2,
            dx: vel.0,
            dy: vel.1,
            dz: vel.2,
        }
    }

    pub fn update_pos(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.z += self.dz;
    }

    pub fn get_pos(&self) -> (i64, i64, i64) {
        (self.x, self.y, self.z)
    }

    pub fn get_state_x(&self) -> (i64, i64) {
        (self.x, self.dx)
    }

    pub fn get_state_y(&self) -> (i64, i64) {
        (self.y, self.dy)
    }

    pub fn get_state_z(&self) -> (i64, i64) {
        (self.z, self.dz)
    }

    pub fn apply_delta(&mut self, delta: (i64, i64, i64)) {
        self.dx += delta.0;
        self.dy += delta.1;
        self.dz += delta.2;
    }

    pub fn get_energy(&self) -> i64 {
        let potential = self.x.abs() + self.y.abs() + self.z.abs();
        let kinetic = self.dx.abs() + self.dy.abs() + self.dz.abs();

        potential * kinetic
    }
}
