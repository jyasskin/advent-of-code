use std::cmp::max;
use std::io;
use std::io::BufRead;

fn main() {
    let mut total_fuel = 0;
    for line in io::stdin().lock().lines() {
        let mass: i32 = line
            .expect("Can't get lines from stdin")
            .parse()
            .expect("Line wasn't an integer");
        total_fuel += fuel_for_mass(mass);
    }
    println!("{}", total_fuel);
}

fn fuel_for_mass(mass: i32) -> i32 {
    let mut fuel = 0;
    let mut incremental_fuel = mass;
    loop {
        incremental_fuel = max(incremental_fuel / 3 - 2, 0);
        if incremental_fuel == 0 {
            break;
        }
        fuel += incremental_fuel;
    }
    return fuel;
}
