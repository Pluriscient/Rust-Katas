fn next_smaller_number(n: u64) -> Option<u64> {
    println!("{}", n);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_smaller_tests() {
        assert_eq!(Some(12), next_smaller_number(21));
        assert_eq!(Some(790), next_smaller_number(907));
        assert_eq!(Some(513), next_smaller_number(531));
        assert_eq!(None, next_smaller_number(1027));
        assert_eq!(Some(414), next_smaller_number(441));
    }
}
