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
    let mut has_isolated_double = false;
    for next in 1..6 {
        if digits[next] == digits[next - 1] // In a pair.
            && (next == 1 || digits[next] != digits[next - 2]) // Different from digit before the pair.
            && (next == 5 || digits[next] != digits[next + 1])
        // Different from digit after the pair.
        {
            has_isolated_double = true;
        }
        if digits[next] < digits[next - 1] {
            return false;
        }
    }
    return has_isolated_double;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn examples() {
        assert!(validate(112233));
        assert!(!validate(123444));
        assert!(validate(111122));
    }
}
