use std::collections::HashMap;
use std::convert::TryInto;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read input");
    let lines: Vec<&str> = input.split('\n').collect();

    println!("Part 1: {}", count_orbits(&compute_orbits(&lines)));
    println!("Part 2: {}", you2san(&compute_orbits(&lines)));
}

// key orbits around value
fn compute_orbits(lines: &[&str]) -> HashMap<String, String> {
    let mut orbits: HashMap<String, String> = HashMap::new();
    for line in lines {
        let orbit: Vec<&str> = line.split(')').collect();
        if orbit.len() != 2 {
            break;
        }
        orbits.insert(orbit[1].to_string(), orbit[0].to_string());
    }
    return orbits;
}

fn count_orbits(orbits: &HashMap<String, String>) -> i64 {
    let mut total_orbits = 0i64;
    for (satellite, _) in orbits {
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

fn sat2com<'a>(orbits: &'a HashMap<String, String>, satellite: &'a str) -> Vec<&'a str> {
    let mut result = vec![satellite];
    loop {
        match orbits.get(*result.last().unwrap()) {
            Some(center) => result.push(center),
            None => break,
        }
    }
    result
}

fn you2san(orbits: &HashMap<String, String>) -> i64 {
    let mut you2com = sat2com(orbits, "YOU");
    let mut san2com = sat2com(orbits, "SAN");
    you2com.reverse();
    san2com.reverse();
    let mut i = 0usize;
    loop {
        if you2com[i] != san2com[i] {
            return (you2com.len() - 1 - i + san2com.len() - 1 - i)
                .try_into()
                .unwrap();
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!(count_orbits(&compute_orbits(&["COM)B", "B)C"])), 3);
        assert_eq!(
            count_orbits(&compute_orbits(&[
                "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L"
            ])),
            42
        );
        assert_eq!(
            you2san(&compute_orbits(&[
                "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
                "K)YOU", "I)SAN"
            ])),
            4
        )
    }
}
