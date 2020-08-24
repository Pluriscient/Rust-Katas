fn indices(n: u32, d: u32) -> Vec<Vec<u32>> {
    recurse(n, d, vec![])
}

fn recurse(n: u32, d: u32, mut prefix: Vec<u32>) -> Vec<Vec<u32>> {
    if n == 1 {
        prefix.push(d);
        vec![prefix]
    } else {
        let mut res = Vec::with_capacity((d.pow(n as u32)) as usize);
        for i in 0..=d {
            let mut new_prefix = prefix.clone();
            new_prefix.push(i);
            res.extend(recurse(n - 1, d - i, new_prefix))
        }
        res
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;
    use super::*;
    use rand::distributions::{Distribution, Uniform};

    fn sorted_equal(mut actual: Vec<Vec<u32>>, expected: Vec<Vec<u32>>) {
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_basic() {
        let mut actual = indices(3, 2);
        actual.sort();
        assert_eq!(
            actual,
            [
                [0, 0, 2],
                [0, 1, 1],
                [0, 2, 0],
                [1, 0, 1],
                [1, 1, 0],
                [2, 0, 0]
            ]
        )
    }

    #[test]
    fn test_quartic_triangles() {
        let mut actual = indices(3, 4);
        actual.sort();
        assert_eq!(
            actual,
            [
                [0, 0, 4],
                [0, 1, 3],
                [0, 2, 2],
                [0, 3, 1],
                [0, 4, 0],
                [1, 0, 3],
                [1, 1, 2],
                [1, 2, 1],
                [1, 3, 0],
                [2, 0, 2],
                [2, 1, 1],
                [2, 2, 0],
                [3, 0, 1],
                [3, 1, 0],
                [4, 0, 0]
            ]
        )
    }
    fn solution(n: u32, d: u32, mut prefix: Vec<u32>) -> Vec<Vec<u32>> {
        if n == 1 {
            prefix.push(d);
            vec![prefix]
        } else {
            let mut res = Vec::with_capacity((d.pow(n as u32)) as usize);
            for i in 0..=d {
                let mut new_prefix = prefix.clone();
                new_prefix.push(i);
                res.extend(solution(n - 1, d - i, new_prefix))
            }
            res
        }
    }

    #[test]
    fn test_10() {
        let n = 10;
        let d = 5;
        let mut expected = solution(n, d, Vec::with_capacity(n as usize));
        expected.sort();
        let mut actual = indices(n, d);
        actual.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_random() {
        let mut rng = rand::thread_rng();
        let between = Uniform::from(3..9);

        for _ in 0..50 {
            let n = between.sample(&mut rng);
            let d = between.sample(&mut rng);
            let mut expected = solution(n, d, Vec::with_capacity(n as usize));
            expected.sort();
            let mut actual = indices(n, d);
            actual.sort();
            assert_eq!(actual, expected);
        }
    }
}
