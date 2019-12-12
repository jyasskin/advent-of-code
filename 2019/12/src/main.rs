use std::cmp::Ordering;
use std::io;
use std::io::Read;

fn main() {
    let moons = vec![
        moon(-4, -9, -3, 0, 0, 0),
        moon(-13, -11, 0, 0, 0, 0),
        moon(-17, -7, 15, 0, 0, 0),
        moon(-16, 4, 2, 0, 0, 0),
    ];

    let steps = simulate_moons(moons.clone(), 1000);
    let step_1000 = &steps[1000];
    let total_energy: i32 = step_1000.iter().map(|moon| moon.energy()).sum();
    println!("Part 1: {:?}, Total Energy: {}", step_1000, total_energy);
}

#[derive(Default, PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    vx: i32,
    vy: i32,
    vz: i32,
}
impl Moon {
    fn energy(&self) -> i32 {
        (self.x.abs() + self.y.abs() + self.z.abs())
            * (self.vx.abs() + self.vy.abs() + self.vz.abs())
    }
}
fn moon(x: i32, y: i32, z: i32, vx: i32, vy: i32, vz: i32) -> Moon {
    Moon {
        x,
        y,
        z,
        vx,
        vy,
        vz,
    }
}

fn _input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read input");
    input
}

fn simulate_moons(mut moons: Vec<Moon>, steps: i32) -> Vec<Vec<Moon>> {
    let mut result = Vec::new();
    for _ in 0..steps {
        result.push(moons.clone());
        update_velocities(&mut moons);
        for mut moon in &mut moons {
            update_position(&mut moon);
        }
    }
    result.push(moons.clone());
    result
}

fn update_velocities(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        for j in 0..moons.len() {
            if i == j {
                continue;
            }
            update_velocity(moons[i].x, moons[j].x, &mut moons[i].vx);
            update_velocity(moons[i].y, moons[j].y, &mut moons[i].vy);
            update_velocity(moons[i].z, moons[j].z, &mut moons[i].vz);
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

fn update_position(moon: &mut Moon) {
    moon.x += moon.vx;
    moon.y += moon.vy;
    moon.z += moon.vz;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example1() {
        let moons = vec![
            Moon {
                x: -1,
                y: 0,
                z: 2,
                ..Default::default()
            },
            Moon {
                x: 2,
                y: -10,
                z: -7,
                ..Default::default()
            },
            Moon {
                x: 4,
                y: -8,
                z: 8,
                ..Default::default()
            },
            Moon {
                x: 3,
                y: 5,
                z: -1,
                ..Default::default()
            },
        ];
        let steps = simulate_moons(moons, 10);
        assert_eq!(
            steps[1],
            vec![
                moon(2, -1, 1, 3, -1, -1),
                moon(3, -7, -4, 1, 3, 3),
                moon(1, -7, 5, -3, 1, -3),
                moon(2, 2, 0, -1, -3, 1),
            ]
        );
        assert_eq!(
            steps[10],
            vec![
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
            vec![
                moon(-8, -10, 0, 0, 0, 0),
                moon(5, 5, 10, 0, 0, 0),
                moon(2, -7, 3, 0, 0, 0),
                moon(9, -8, -3, 0, 0, 0),
            ],
            100,
        );
        assert_eq!(
            steps[100],
            vec![
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
}
