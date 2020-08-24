use crate::utils::primes::is_prime;
use std::str::FromStr;

fn backwards_prime(start: u64, stop: u64) -> Vec<u64> {
    let mut res = vec![];
    for i in start..=stop {
        if is_prime(i as usize) {
            let ri =
                u64::from_str(i.to_string().chars().rev().collect::<String>().as_str()).unwrap();
            println!("rev of {} is {}", i, ri);
            if ri != i && is_prime(ri as usize) {
                res.push(i);
            }
        }
    }

    res
}

fn testing(start: u64, stop: u64, exp: Vec<u64>) {
    assert_eq!(backwards_prime(start, stop), exp)
}

#[test]
fn tests_backwards_prime() {
    let a = vec![13, 17, 31, 37, 71, 73, 79, 97];
    testing(1, 100, a);
    let a = vec![13, 17, 31];
    testing(1, 31, a);
}
