//https://www.codewars.com/kata/55cf3b567fc0e02b0b00000b
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn part(n: i64) -> String {
    // part_naive(n as u64)
    part_map(n as u64)
}

fn part_naive(n: u64) -> String {
    //recursion ftw
    let mut parts = partitions(n);
    println!("parts: {:?}", parts);
    parts.sort();
    parts.dedup();
    println!("look: {:?}", parts);
    let mut prods: Vec<u64> = parts.iter().map(|x| x.iter().product()).collect();
    prods.sort_unstable();
    prods.dedup();
    println!("prods: {:?}", prods);

    let range = prods.iter().max().unwrap() - prods.iter().min().unwrap();
    let avg = prods.iter().sum::<u64>() as f64 / prods.len() as f64;
    let median = if prods.len() % 2 == 0 {
        (prods[prods.len() / 2] + prods[(prods.len() / 2) - 1]) as f64 / 2f64
    } else {
        prods[prods.len() / 2] as f64
    };

    format!("Range: {} Average: {:.2} Median: {:.2}", range, avg, median)
}

fn part_set(n: u64) -> String {
    let mut mem: HashSet<Vec<u64>> = HashSet::new();

    partitions_set(n, vec![], &mut mem);
    println!("mem: {:?}", mem);
    let prods: HashSet<u64> = HashSet::from_iter(mem.iter().map(|x| x.iter().product()));
    let range = prods.iter().max().unwrap() - 1;
    let avg = prods.iter().sum::<u64>() as f64 / prods.len() as f64;
    let mut prods: Vec<u64> = prods.into_iter().collect();
    prods.sort_unstable();
    let median = if prods.len() % 2 == 0 {
        (prods[prods.len() / 2] + prods[(prods.len() / 2) - 1]) as f64 / 2f64
    } else {
        prods[prods.len() / 2] as f64
    };
    format!("Range: {} Average: {:.2} Median: {:.2}", range, avg, median)
}

// !!WORKING!!
fn part_map(n: u64) -> String {
    let mut mem: HashMap<u64, HashSet<u64>> = HashMap::with_capacity((n) as usize);
    partitions_map(n, &mut mem);
    let mut prods: Vec<u64> = mem.get(&n).unwrap().clone().into_iter().collect();

    prods.sort_unstable();
    let range = prods.iter().max().unwrap() - 1;
    let avg = prods.iter().sum::<u64>() as f64 / prods.len() as f64;
    let median = if prods.len() % 2 == 0 {
        (prods[prods.len() / 2] + prods[(prods.len() / 2) - 1]) as f64 / 2f64
    } else {
        prods[prods.len() / 2] as f64
    };
    format!("Range: {} Average: {:.2} Median: {:.2}", range, avg, median)
}

fn partitions_set(n: u64, prefix: Vec<u64>, mem: &mut HashSet<Vec<u64>>) {
    let new_prefix = {
        let mut p = prefix.clone();
        p.push(n);
        p
    };
    if mem.insert(new_prefix) {
        for i in 1..n {
            let new_prefix = {
                let mut p = prefix.clone();
                p.push(i);
                p
            };
            partitions_set(n - i, new_prefix, mem)
        }
    } else {
        println!("skipping...{} ;  {:?}", n, prefix);
    }
}

fn partitions_map(n: u64, mem: &mut HashMap<u64, HashSet<u64>>) {
    // just in case you know
    if mem.contains_key(&n) {
        return;
    }

    let mut res = HashSet::with_capacity(n as usize);
    res.insert(n);

    for i in 1..n {
        // memoization of the next ones
        if !mem.contains_key(&(n - i)) {
            println!("{} was not in cache", n - i);
            partitions_map(n - i, mem);
        }
        // as we only have to store the unique products just use those
        let parts = mem.get(&(n - i)).unwrap();
        res.extend(parts.iter().map(|x| x * i));
    }
    mem.insert(n, res);
}

//clever one;
fn sub(n: i64, m: i64) -> Vec<i64> {
    if m == 1 {
        return vec![1];
    }
    (0..=n / m)
        .flat_map(|p| {
            let pow = m.pow(p as u32);
            sub(n - m * p, m - 1)
                .iter()
                .map(|s| s * pow)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part_web(n: i64) -> String {
    let mut v = sub(n, n);
    v.sort_unstable();
    v.dedup();
    let len = v.len();
    let rng = v[len - 1] - v[0];
    let avg = v.iter().sum::<i64>() as f64 / len as f64;
    let med = if len % 2 == 0 {
        (v[len / 2 - 1] + v[len / 2]) as f64 / 2.0
    } else {
        v[len / 2] as f64
    };
    format!("Range: {} Average: {:.02} Median: {:.02}", rng, avg, med)
}

//recursion will be better here
fn partitions(n: u64) -> Vec<Vec<u64>> {
    let mut res = vec![vec![n]];
    // print!("Start for {}\t", n);
    for i in 1..n {
        let mut part = partitions(n - i);
        // println!("Getting {:?}", part);
        part.iter_mut().for_each(|x| x.push(i));
        // println!("transformed to {:?}", part);

        res.extend(part);
    }
    // println!("res for {} is {:?}", n, res);
    res
}

fn test_equal(ans: &str, sol: &str) {
    assert_eq!(ans, sol, "Expected \"{}\", got \"{}\".", sol, ans);
}

#[test]
fn partitions_test() {
    test_equal(&part(1), "Range: 0 Average: 1.00 Median: 1.00");
    test_equal(&part(2), "Range: 1 Average: 1.50 Median: 1.50");
    test_equal(&part(3), "Range: 2 Average: 2.00 Median: 2.00");
    test_equal(&part(4), "Range: 3 Average: 2.50 Median: 2.50");
    // println!("starting test");
    test_equal(&part(5), "Range: 5 Average: 3.50 Median: 3.50");
}
