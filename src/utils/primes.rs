extern crate bit_vec;

use self::bit_vec::BitVec;

pub fn euclidean_sieve(end: usize) -> BitVec {
    let mut bv = BitVec::from_elem(end + 1, true);
    bv.set(0, false);
    bv.set(1, false);
    let sqrt_n = ((end as f64).sqrt() as usize) + 1;
    for i in 2..sqrt_n {
        if bv[i] {
            // let prod = i * j;
            let max_j = end / i;
            for j in i..=max_j {
                bv.set(i * j, false);
            }
        }
    }
    bv
}

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

#[cfg(test)]
mod test {
    use crate::utils::primes::{euclidean_sieve, is_prime};

    #[test]
    fn check_first_few_thousand() {
        let sieve = euclidean_sieve(1000);
        let primes: Vec<usize> = sieve
            .into_iter()
            .enumerate()
            .filter(|(_, b)| *b)
            .map(|x| x.0)
            .collect();
        assert!(primes.iter().all(|x| is_prime(*x)));
        // assert_eq!(primes, [2, 3, 5, 7]);
    }
}
