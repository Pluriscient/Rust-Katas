fn doubles(maxk: i32, maxn: i32) -> f64 {
    fn v(k: i32, n: i32) -> f64 {
        let n = f64::from(n);
        return 1f64 / (k as f64 * (n + 1f64).powi(2 * k));
    }
    fn u(k: i32, max_n: i32) -> f64 {
        (1..=max_n).map(|n| v(k, n)).sum()
    }
    (1..=maxk).map(|k| u(k, maxn)).sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    fn assert_fuzzy_equals(actual: f64, expected: f64) {
        let merr = 1.0e-10;
        let inrange = if expected == 0.0 {
            actual.abs() <= merr
        } else {
            (actual - expected).abs() / expected <= merr
        };
        assert!(
            inrange,
            format!(
                "Expected value must be near: {:e} but was:{:e}",
                expected, actual
            )
        );
    }

    fn dotest(maxk: i32, maxn: i32, exp: f64) -> () {
        assert_fuzzy_equals(doubles(maxk, maxn), exp);
    }

    #[test]
    fn basic_tests_doubles() {
        dotest(1, 10, 0.5580321939764581);
        dotest(10, 1000, 0.6921486500921933);
        dotest(10, 10000, 0.6930471674194457);
    }
}
