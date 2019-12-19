pub use euclid;
pub use euclid::point2;
pub use euclid::rect;
pub use euclid::size2;
pub use euclid::vec2;
use itertools::Itertools;
pub use num::rational::Rational32;
use std::fs::File;
use std::io;
use std::io::Read;

pub type Rect = euclid::default::Rect<i64>;
pub type Point2 = euclid::default::Point2D<i64>;
pub type Vector2 = euclid::default::Vector2D<i64>;

pub fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read input");
    input
}

pub fn read_file(name: &str) -> String {
    let mut result = String::new();
    File::open(name)
        .unwrap()
        .read_to_string(&mut result)
        .unwrap();
    result
}

// Returns the in-order traversal of the Stern-Brocot tree down to the layer'th
// layer, which does not include the 0/1 and 1/0 elements at the edges of the
// tree.
//
// This is a sorted list of rational numbers that is a superset of all rationals
// with a non-negative numerator and denominator <= layer.
pub fn stern_brocot_sequence(layer: i32) -> Vec<Rational32> {
    let mut result = vec![Rational32::new_raw(0, 1), Rational32::new_raw(1, 0)];
    for _ in 1..=layer {
        let first = result.iter();
        let mut second = result.iter();
        second.next();
        result =
            result
                .iter()
                .copied()
                .interleave(first.zip(second).map(|(f, s)| {
                    Rational32::new_raw(f.numer() + s.numer(), f.denom() + s.denom())
                }))
                .collect();
    }
    result[1..result.len() - 1].into()
}

pub fn frac(n: i32, d: i32) -> Rational32 {
    Rational32::new(n, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stern_brocot() {
        assert_eq!(stern_brocot_sequence(0), vec![]);
        assert_eq!(stern_brocot_sequence(1), vec![frac(1, 1)]);
        assert_eq!(
            stern_brocot_sequence(2),
            vec![frac(1, 2), frac(1, 1), frac(2, 1)]
        );
        assert_eq!(
            stern_brocot_sequence(3),
            vec![
                frac(1, 3),
                frac(1, 2),
                frac(2, 3),
                frac(1, 1),
                frac(3, 2),
                frac(2, 1),
                frac(3, 1)
            ]
        );

        let sb30 = stern_brocot_sequence(20);
        for i in 1..sb30.len() {
            assert!(sb30[i - 1] < sb30[i]);
        }
    }
}
