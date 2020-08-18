// pub fn run() {}

fn nb_dig(n: i32, d: i32) -> i32 {
    let digit = (('0' as i32 + d) as u8) as char;
    // (0..n+1).for_each(|x| println!("{}", x));
    (0..n + 1)
        .map(|x| x * x)
        // .map(|x|x.to)
        .flat_map(|x| x.to_string().chars().collect::<Vec<char>>())
        .filter(|&c| c == digit)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i32, d: i32, exp: i32) -> () {
        println!("n: {:?}", n);
        println!("d: {:?}", d);
        let ans = nb_dig(n, d);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(550, 5, 213);
        dotest(5750, 0, 4700);
    }
}
