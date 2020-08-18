fn parse(code: &str) -> Vec<i32> {
    let mut mem: Vec<i32> = vec![];
    let mut cur = 0;
    for command in code.chars() {
        match command {
            'i' => cur += 1,
            'd' => cur -= 1,
            's' => cur *= cur,
            'o' => mem.push(cur),
            _ => panic!("unknown command"),
        }
    }
    mem
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_tests() {
        assert_eq!(parse("iiisdoso"), vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
    }
}
