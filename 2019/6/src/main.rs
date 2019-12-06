use std::collections::HashMap;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read input");
    let lines: Vec<&str> = input.split('\n').collect();

    println!("Part 1: {}", count_orbits(&lines));
    //println!("Part 2: {}", part2(&lines));
}

fn count_orbits(lines: &[&str]) -> i64 {
    // key orbits around value
    let mut orbits: HashMap<String, String> = HashMap::new();
    for line in lines {
        let orbit: Vec<&str> = line.split(')').collect();
        if orbit.len() != 2 {
            break;
        }
        orbits.insert(orbit[1].to_string(), orbit[0].to_string());
    }

    let mut total_orbits = 0i64;
    for (satellite, _) in &orbits {
        let mut satellite: &str = &satellite;
        loop {
            match orbits.get(satellite) {
                Some(center) => satellite = center,
                None => break,
            }
            total_orbits += 1;
        }
    }
    return total_orbits;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!(count_orbits(&["COM)B", "B)C"]), 3);
        assert_eq!(
            count_orbits(&[
                "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L"
            ]),
            42
        );
        //assert_eq!(part2(&vec![]), "");
    }
}
