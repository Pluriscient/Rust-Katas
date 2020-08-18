use std::iter::FromIterator;

fn rot13(message: &str) -> String {
    String::from_iter(message.chars().map(|x| {
        if !x.is_ascii_alphabetic() {
            return x;
        } else if x.is_ascii_lowercase() {
            (((x as u8) - ('a' as u8) + 13) % 26 + ('a' as u8)) as char
        } else {
            (((x as u8) - ('A' as u8) + 13) % 26 + ('A' as u8)) as char
        }
    }))
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(rot13("test"), "grfg");
        assert_eq!(rot13("Test"), "Grfg");
    }
}
