#![allow(dead_code)]
mod problems;
mod probs;
mod utils;

// fn dotest(a1: Vec<&str>, a2: Vec<&str>, exp: i32) -> () {
//     println!("a1: {:?};", a1);
//     println!("a2: {:?};", a2);
//     let ans = mx_dif_lg(a1, a2);
//     println!("actual:\n{:?};", ans);
//     println!("expect:\n{:?};", exp);
//     println!("{};", ans == exp);
//     assert_eq!(ans, exp);
//     println!("{};", "-");
// }
fn main() {
    // probs::check_cheating::basics_remove_nb();
    // let s1 = vec![
    //     "hoqq",
    //     "bbllkw",
    //     "oox",
    //     "ejjuyyy",
    //     "plmiis",
    //     "xxxzgpsssa",
    //     "xxwwkktt",
    //     "znnnnfqknaz",
    //     "qqquuhii",
    //     "dvvvwz",
    // ];
    // let s2 = vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"];
    // dotest(s1, s2, 13);
    // assert_eq!(round_to_next_5(-3), 0);
}
//
// fn digits(num: u64) -> Vec<u64> {
//     let base: u64 = 10;
//     let mut x = num;
//     let mut digits: Vec<u64> = vec![];
//     while x >= base {
//         digits.push(x % base);
//         x /= base;
//     }
//     digits.push(x % base);
//     digits.reverse();
//     println!("digits of {}: {:?}", num, digits);
//     return digits;
// }
// fn round_to_next_5(n: i32) -> i32 {
//     println!("{} % 5  == {}", n, n % 5);
//     if n % 5 == 0 {
//         return n;
//     }
//     if n > 0 {
//         return n + 5 - (n % 5);
//     }
//     // return n + ()
//     return n - (n % 5);
//     // let i = n + 5 - (n % 5).abs();
//     // println!("i: {}", 5 - (n % 5).abs());
//     // return i;
// }
// fn nb_dig(n: i32, d: i32) -> i32 {
//     let digit = (('0' as i32 + d) as u8) as char;
//     (0..n)
//         .map(|x| x * x)
//         // .map(|x|x.to)
//         .flat_map(|x| x.to_string().chars().collect::<Vec<char>>())
//         .filter(|&c| c == digit)
//         .count() as i32
// }

// fn mx_dif_lg(a1: Vec<&str>, a2: Vec<&str>) -> i32 {
//     if a1.is_empty() || a2.is_empty() {
//         return -1;
//     }
//     let alengths: Vec<usize> = a1.iter().map(|x| x.len()).collect();
//     let amin = alengths.iter().min().unwrap();
//     let amax = alengths.iter().max().unwrap();
//     let blengths: Vec<usize> = a2.iter().map(|x| x.len()).collect();
//     let bmin = blengths.iter().min().unwrap();
//     let bmax = blengths.iter().max().unwrap();
//     (bmax - amin).max(amax - bmin) as i32
// }

// fn find_short(s: &str) -> u32 {
//     //your code here
//     let x = s
//         .split_whitespace()
//         .min_by(|x, y| x.len().cmp(&y.len()))
//         .unwrap()
//         .len();
//     return x as u32;
// }
//
// fn printer_error(s: &str) -> String {
//     let max_char = 'm';
//     let total_length = s.len();
//     let errors = s.chars().filter(|x| x > &max_char).count();
//
//     return format!("{}/{}", errors, total_length);
// }

// fn persistence(num: u64) -> u64 {
//     let mut count = 0;
//     let mut ds = digits(num);
//     while ds.len() > 1 {
//         let product = ds.iter().product();
//         ds = digits(product);
//         count += 1;
//         println!("entered..")
//     }
//
//     return count;
// }
