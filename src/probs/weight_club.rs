use std::cmp::Ordering;
use std::str::FromStr;

fn order_weight(s: &str) -> String {
    fn compare_fun(x: &usize, &y: &usize) -> Ordering {
        match sum_digits(*x).cmp(&sum_digits(y)) {
            Ordering::Equal => x.to_string().cmp(&y.to_string()),
            res => res,
        }
    }
    fn sum_digits(n: usize) -> usize {
        n.to_string()
            .chars()
            .map(|d| d as u32 - '0' as u32)
            .sum::<u32>() as usize
    }
    let mut weights: Vec<usize> = s
        .split_whitespace()
        .map(|x| usize::from_str(x).unwrap())
        .collect();
    weights.sort_by(compare_fun);
    let words: Vec<String> = weights.iter().map(|x| x.to_string()).collect();
    words.join(" ")
}

fn testing(s: &str, exp: &str) {
    assert_eq!(order_weight(s), exp)
}

#[test]
fn basics_order_weight() {
    testing("103 123 4444 99 2000", "2000 103 123 4444 99");
    testing(
        "2000 10003 1234000 44444444 9999 11 11 22 123",
        "11 11 2000 10003 22 123 1234000 44444444 9999",
    );
}
