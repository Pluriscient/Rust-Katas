fn print(n: i32) -> Option<String> {
    if n < 0 || n % 2 == 0 {
        return None;
    }
    let maxi = (n / 2) + 1;
    let mut range = (1..maxi).collect::<Vec<i32>>();
    let mut down_range = (1..maxi + 1).rev().collect::<Vec<i32>>();
    range.append(&mut down_range);
    println!("{}range: {:?}", maxi, range);

    let half: Vec<String> = range
        .iter()
        .map(|x| {
            let space_round: String = (0..(maxi - x)).map(|_| " ").collect();
            let middle: String = (0..(n - (2 * space_round.len() as i32)))
                .map(|_| "*")
                .collect();

            space_round + middle.as_str()
            // space_round + middle + space_round
        })
        .collect();

    Some(half.join("\n") + "\n")
}

fn nicer(n: i32) -> Option<String> {
    if n < 0 || n % 2 == 0 {
        return None;
    }

    let n = n as usize;
    let diamond = (1..=n)
        .chain((1..n).rev())
        .step_by(2)
        .map(|i| format!("{}{}\n", " ".repeat((n - i) / 2), "*".repeat(i)))
        .collect();

    Some(diamond)
}

#[test]
fn basic_test() {
    // assert_eq!(print(3), Some(" *\n***\n *\n".to_string()));
    assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()));
    assert_eq!(nicer(-3), None);
    assert_eq!(print(2), None);
    assert_eq!(print(0), None);
    assert_eq!(print(1), Some("*\n".to_string()));
}
