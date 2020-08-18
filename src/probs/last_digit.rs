use std::str::FromStr;

fn last_digit_naive(str1: &str, str2: &str) -> i32 {
    let n: i32 = (str1.as_bytes()[str1.len() - 1] as u8 - '0' as u8) as i32;
    let p: i32 = (str2.as_bytes()[str2.len() - 1] as u8 - '0' as u8) as i32;
    println!("{} ^ {}", n, p);
    if n == 0 {
        0
    } else {
        n.pow(p as u32) % 10
    }
}

fn last_digit(str1: &str, str2: &str) -> i32 {
    let n: i32 = i32::from_str(&str1[if str1.len() > 1 { str1.len() - 2 } else { 0 }..]).unwrap();
    let p: i32 = i32::from_str(&str2[if str2.len() > 1 { str2.len() - 2 } else { 0 }..]).unwrap();

    println!("{} ^ {}", n, p);
    if n % 10 == 0 {
        0
    } else {
        n.wrapping_pow(p as u32) % 10
    }
}
#[test]
fn returns_expected() {
    assert_eq!(last_digit("4", "1"), 4);
    assert_eq!(last_digit("4", "2"), 6);
    assert_eq!(last_digit("9", "7"), 9);
    assert_eq!(last_digit("10", "10000000000"), 0);
    assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376","2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
    assert_eq!(
        last_digit(
            "3715290469715693021198967285016729344580685479654510946723",
            "68819615221552997273737174557165657483427362207517952651"
        ),
        7
    );
}
