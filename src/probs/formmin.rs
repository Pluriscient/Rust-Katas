use std::str::FromStr;

fn min_value(mut digits: Vec<i32>) -> i32 {
    digits.sort_unstable();
    digits.dedup();
    let string = digits
        .iter()
        .fold(String::from(""), |x, y| x + y.to_string().as_str());
    // println!("{} ", string);

    i32::from_str(string.as_str()).unwrap()
}

#[test]
fn basic_test() {
    assert_eq!(min_value(vec![1, 3, 1]), 13);
    assert_eq!(min_value(vec![4, 7, 5, 7]), 457);
    assert_eq!(min_value(vec![4, 8, 1, 4]), 148);
}
