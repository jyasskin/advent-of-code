fn main() {
    let mut count = 0;
    for password in 347312..805916 {
        if validate(password) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn validate(password: i32) -> bool {
    let digits: Vec<char> = format!("{}", password).chars().collect();
    let mut has_double = false;
    let mut prev = digits[0];
    for next in &digits[1..] {
        if *next == prev {
            has_double = true;
        }
        if *next < prev {
            return false;
        }
        prev = *next;
    }
    return has_double;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert!(validate(111111));
        assert!(!validate(223450));
        assert!(!validate(123789));
    }
}
