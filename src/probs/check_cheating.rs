extern crate indicatif;

fn remove_nb_unop(m: i32) -> Vec<(i32, i32)> {
    let mut res = vec![];

    for a in 1..m {
        for b in a + 1..=m {
            if a * b == (1..=m).filter(|i| i != &a && i != &b).sum() {
                res.push((a, b));
                res.push((b, a))
            }
        }
    }
    res.sort_by_key(|x| x.0);
    res
}

pub fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    let mut res: Vec<(i64, i64)> = vec![];
    let m = m as i64;
    let sum = (m * (m + 1)) / 2;
    // for a in (m / 2)..(4 * m / 5) {
    for a in m / 2..m {
        let b = (m * (m + 1) - 2 * a) / (2 * a + 1);

        if a * b == (sum - a - b) {
            println!("Found: ({}, {})", a, b);
            res.push((a, b));
            // res.push((b, a))
        }
    }
    res.sort_by_key(|x| x.0);
    res.iter().map(|x| (x.0 as i32, x.1 as i32)).collect()
}

fn testing(n: i32, exp: Vec<(i32, i32)>) -> () {
    assert_eq!(remove_nb(n), exp)
}

#[test]
pub fn basics_remove_nb() {
    // testing(26, vec![(15, 21), (21, 15)]);
    // testing(100, vec![]);
    // testing(101, vec![(55, 91), (91, 55)]);
    // testing(102, vec![(70, 73), (73, 70)]);
    // testing(103, vec![]);
    // testing(1006, vec![(546, 925), (925, 546)]);
    // testing(846, vec![(498, 717), (717, 498)]);
    testing(
        1000003,
        vec![
            (550320, 908566),
            (559756, 893250),
            (893250, 559756),
            (908566, 550320),
        ],
    );
}
