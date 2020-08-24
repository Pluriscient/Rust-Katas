use std::iter::FromIterator;

fn rot13(message: &str) -> String {
    String::from_iter(message.chars().map(|x| {
        if !x.is_ascii_alphabetic() {
            x
        } else if x.is_ascii_lowercase() {
            (((x as u8) - (b'a') + 13) % 26 + (b'a')) as char
        } else {
            (((x as u8) - (b'A') + 13) % 26 + (b'A')) as char
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
