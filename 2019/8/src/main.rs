use std::io;
use std::io::BufRead;

fn main() {
    let lines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .expect("Couldn't read input");
    let pixels: Vec<u32> = lines[0].chars().map(|n| n.to_digit(10).unwrap()).collect();
    type Layer = [[u32; 25]; 6];
    let num_layers = pixels.len() / 25 / 6;
    assert_eq!(num_layers * 25 * 6, pixels.len());
    let mut layers: Vec<Layer> = vec![];
    let mut iter = pixels.into_iter();
    let mut min_zeros = i32::max_value();
    let mut score = 0i32;
    for _ in 0..num_layers {
        let mut layer: Layer = [[0; 25]; 6];
        let mut zeros = 0i32;
        let mut ones = 0i32;
        let mut twos = 0i32;
        for row in 0..6 {
            for col in 0..25 {
                layer[row][col] = iter.next().unwrap();
                match layer[row][col] {
                    0 => zeros += 1,
                    1 => ones += 1,
                    2 => twos += 1,
                    _ => {}
                };
            }
        }
        if zeros < min_zeros {
            min_zeros = zeros;
            score = ones * twos;
        }
        layers.push(layer);
    }
    println!("Part 1: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert_eq!("", "");
    }
}
