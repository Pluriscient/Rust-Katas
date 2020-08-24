use std::collections::{HashMap, HashSet};

pub fn is_prime(n: usize) -> bool {
    match n {
        1 => false,
        2 | 3 => true,
        x if x % 2 == 0 || x % 3 == 0 => false,
        _ => {
            let sqrt_n = ((n as f64).sqrt() as usize) + 1;
            !(6..=sqrt_n)
                .step_by(6)
                .any(|i| n % (i - 1) == 0 || n % (i + 1) == 0)
        }
    }
}

fn prime_factors(mut n: i64) -> Vec<i64> {
    n = n.abs();
    if n == 1 || n == 2 || n == 3 {
        return vec![];
    }

    let mut res = HashSet::new(); //todo initial capacity?
    if n % 2 == 0 {
        res.insert(2);
        n /= 2;
        while n % 2 == 0 {
            n /= 2
        }
    }
    if n % 3 == 0 {
        res.insert(3);
        while n % 3 == 0 {
            n /= 3
        }
    }
    // let mut sqrt_n: i64 = ((n as f64).sqrt() as i64) + 1;
    let mut i = 6;
    while i < n {
        if n % (i - 1) == 0 {
            res.insert(i - 1);
            n /= i - 1;
            while n % (i - 1) == 0 {
                n /= i - 1;
            }

            // sqrt_n = ((n as f64).sqrt() as i64) + 1;
        }
        if n % (i + 1) == 0 {
            res.insert(i + 1);
            n /= i + 1;
            while n % (i + 1) == 0 {
                n /= i + 1;
            }
            // sqrt_n = ((n as f64).sqrt() as i64) + 1;
        }
        i += 6;
    }
    if n != 1 {
        res.insert(n);
    }
    res.into_iter().collect()
}

fn sum_of_divided(l: Vec<i64>) -> Vec<(i64, i64)> {
    let mut map = HashMap::new();
    for item in l {
        let factors: Vec<i64> = prime_factors(item);
        for factor in factors {
            map.entry(factor).and_modify(|x| *x += item).or_insert(item);
        }
    }

    let mut res: Vec<(i64, i64)> = map.into_iter().collect();
    res.sort_by_key(|x| x.0);
    res
}

#[cfg(test)]
mod factor_tests {
    use super::*;
    #[test]
    fn prime_factor_test_basic() {
        let mut actual = prime_factors(600851475143);
        actual.sort_unstable();
        assert_eq!(actual, [71, 839, 1471, 6857])
    }

    #[test]
    fn test_large_n() {
        assert_eq!(prime_factors(715140684628284199), [845037439, 846282841])
    }

    fn testing(l: Vec<i64>, exp: Vec<(i64, i64)>) -> () {
        assert_eq!(sum_of_divided(l), exp)
    }

    #[test]
    fn basics_sum_of_divided() {
        testing(vec![12, 15], vec![(2, 12), (3, 27), (5, 15)]);
        testing(
            vec![15, 21, 24, 30, 45],
            vec![(2, 54), (3, 135), (5, 90), (7, 21)],
        );
    }
}
