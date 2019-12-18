use num::Integer;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io;
use std::io::Read;
use std::time::Instant;

fn main() {
    let moons = [
        moon(-4, -9, -3, 0, 0, 0),
        moon(-13, -11, 0, 0, 0, 0),
        moon(-17, -7, 15, 0, 0, 0),
        moon(-16, 4, 2, 0, 0, 0),
    ];

    let steps = simulate_moons(moons.clone(), 1000);
    let step_1000 = &steps[1000];
    let total_energy: i32 = step_1000.iter().map(|moon| moon.energy()).sum();
    println!("Part 1: {:?}, Total Energy: {}", step_1000, total_energy);

    println!("Part 2: Repeated on step {}", time_to_repeat(moons))
}

fn time_to_repeat(moons: [Moon; 4]) -> i64 {
    (0..3)
        .map(|axis| {
            time_to_repeat_axis(
                axis,
                moons.into_iter().map(|Moon(moon)| moon[axis]).collect(),
            )
        })
        .fold(1, |a, b| a.lcm(&b))
}

fn time_to_repeat_axis(axis: usize, mut moons: Vec<MoonAxis>) -> i64 {
    let start = Instant::now();
    let mut seen: HashSet<Vec<MoonAxis>> = HashSet::new();
    let mut count: i64 = 0;
    seen.insert(moons.clone());
    loop {
        count += 1;
        step_axis(moons.iter_mut().collect());
        if !seen.insert(moons.clone()) {
            break;
        }
        let lg2 = (count as f64).log2();
        if lg2 == lg2.round() {
            println!("{}; {:?}", count, start.elapsed());
        }
    }
    println!("Axis {} repeated on step {}", axis, count);
    count
}

// Represents a moon's position and velocity along one axis.
#[derive(Default, PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct MoonAxis {
    pos: i32,
    vel: i32,
}
fn axis(pos: i32, vel: i32) -> MoonAxis {
    MoonAxis { pos, vel }
}

#[derive(Default, PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Moon([MoonAxis; 3]);
impl Moon {
    fn x(&self) -> &MoonAxis {
        &self.0[0]
    }
    fn y(&self) -> &MoonAxis {
        &self.0[1]
    }
    fn z(&self) -> &MoonAxis {
        &self.0[2]
    }
    fn energy(&self) -> i32 {
        (self.x().pos.abs() + self.y().pos.abs() + self.z().pos.abs())
            * (self.x().vel.abs() + self.y().vel.abs() + self.z().vel.abs())
    }
}
fn moon(x: i32, y: i32, z: i32, vx: i32, vy: i32, vz: i32) -> Moon {
    Moon([axis(x, vx), axis(y, vy), axis(z, vz)])
}

fn _input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read input");
    input
}

fn simulate_moons(mut moons: [Moon; 4], steps: i32) -> Vec<[Moon; 4]> {
    let mut result = Vec::new();
    for _ in 0..steps {
        result.push(moons.clone());
        moons = step(moons);
    }
    result.push(moons.clone());
    result
}

fn step(mut moons: [Moon; 4]) -> [Moon; 4] {
    for axis in 0..3 {
        step_axis(moons.iter_mut().map(|Moon(moon)| &mut moon[axis]).collect());
    }
    moons
}
fn step_axis(mut moons: Vec<&mut MoonAxis>) {
    update_velocities(&mut moons);
    for moon in moons {
        update_position(moon);
    }
}

fn update_velocities(moons: &mut Vec<&mut MoonAxis>) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if i == j {
                continue;
            }
            update_velocity(moons[i].pos, moons[j].pos, &mut moons[i].vel);
        }
    }
}

fn update_velocity(moon_axis: i32, other_axis: i32, v: &mut i32) {
    match moon_axis.cmp(&other_axis) {
        Ordering::Less => *v += 1,
        Ordering::Equal => {}
        Ordering::Greater => *v -= 1,
    }
}

fn update_position(moon_axis: &mut MoonAxis) {
    moon_axis.pos += moon_axis.vel;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let moons = [
            moon(-1, 0, 2, 0, 0, 0),
            moon(2, -10, -7, 0, 0, 0),
            moon(4, -8, 8, 0, 0, 0),
            moon(3, 5, -1, 0, 0, 0),
        ];
        let steps = simulate_moons(moons, 10);
        assert_eq!(
            steps[1],
            [
                moon(2, -1, 1, 3, -1, -1),
                moon(3, -7, -4, 1, 3, 3),
                moon(1, -7, 5, -3, 1, -3),
                moon(2, 2, 0, -1, -3, 1),
            ]
        );
        assert_eq!(
            steps[10],
            [
                moon(2, 1, -3, -3, -2, 1),
                moon(1, -8, 0, -1, 1, 3),
                moon(3, -6, 1, 3, 2, -3),
                moon(2, 0, 4, 1, -1, -1),
            ]
        );
    }

    #[test]
    fn energy() {
        assert_eq!(moon(2, 1, -3, -3, -2, 1).energy(), 36);
        assert_eq!(moon(16, -13, 23, 7, 1, 1).energy(), 468);
    }

    #[test]
    fn example2() {
        let steps = simulate_moons(
            [
                moon(-8, -10, 0, 0, 0, 0),
                moon(5, 5, 10, 0, 0, 0),
                moon(2, -7, 3, 0, 0, 0),
                moon(9, -8, -3, 0, 0, 0),
            ],
            100,
        );
        assert_eq!(
            steps[100],
            [
                moon(8, -12, -9, -7, 3, 0),
                moon(13, 16, -3, 3, -11, -5),
                moon(-29, -11, -1, -3, 7, 4),
                moon(16, -13, 23, 7, 1, 1)
            ]
        );
        assert_eq!(
            steps[100].iter().map(|moon| moon.energy()).sum::<i32>(),
            1940
        );
    }

    #[test]
    fn repeat_1() {
        assert_eq!(
            time_to_repeat([
                moon(-1, 0, 2, 0, 0, 0),
                moon(2, -10, -7, 0, 0, 0),
                moon(4, -8, 8, 0, 0, 0),
                moon(3, 5, -1, 0, 0, 0)
            ]),
            2772
        );
    }
}
