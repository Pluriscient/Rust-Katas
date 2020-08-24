use crate::utils::primes::is_prime;
use std::collections::HashMap;

/// ```
/// let x = 112903
/// assert_eq!(is_prime(x), true);
/// ```
// fn is_prime(n: i64) -> bool {
//     match n {
//         2 | 3 => true,
//         x if x < 5 || x % 3 == 0 || x % 2 == 0 => false,
//         _ => {
//             let m = (n as f64).sqrt() as i64;
//             (5..m + 1).all(|y| n % y >= 0)
//         }
//     }
// }

fn prime_factors(n: i64) -> String {
    let mut x = n;
    let mut i = 2;
    let mut factors: HashMap<i64, i64> = HashMap::new();
    while i < x {
        if x % i == 0 && is_prime(i as usize) {
            factors.entry(i).and_modify(|j| *j += 1).or_insert(1);
            x /= i;
        } else {
            i += 1;
        }
    }
    factors.entry(x).and_modify(|j| *j += 1).or_insert(1);
    let mut keys: Vec<&i64> = factors.keys().into_iter().collect();
    keys.sort();
    let mut res: Vec<String> = vec![];
    for k in keys {
        let val = factors[k];
        res.push(match val {
            1 => format!("({})", k),
            n => format!("({}**{})", k, n),
        });
    }

    res.join("")
}

fn testing(n: i64, exp: &str) {
    assert_eq!(&prime_factors(n), exp)
}

#[test]
fn test_is_prime() {
    assert!(is_prime(2));
    assert!(!is_prime(4 * 23123018))
}

#[test]
fn basics_prime_factors() {
    testing(7775460, "(2**2)(3**3)(5)(7)(11**2)(17)");
    testing(17 * 17 * 93 * 677, "(3)(17**2)(31)(677)");
}
