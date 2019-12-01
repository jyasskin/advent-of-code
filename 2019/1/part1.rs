use std::io;
use std::io::BufRead;

fn main() {
    let mut total_fuel = 0;
    for line in io::stdin().lock().lines() {
        let mass: u32 = line
            .expect("Can't get lines from stdin")
            .parse()
            .expect("Line wasn't an integer");
        let fuel = mass / 3 - 2;
        total_fuel += fuel;
    }
    println!("{}", total_fuel);
}
