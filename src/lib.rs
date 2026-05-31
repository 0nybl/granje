//! granje

/// Adds two numbers.
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// Multiplies two numbers.
pub fn mul(a: i64, b: i64) -> i64 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn muls() {
        assert_eq!(mul(3, 4), 12);
    }
}
