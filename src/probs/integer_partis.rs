use std::collections::hash_map::Entry;
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
    prods.sort();
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
    let prods: HashSet<u64> =
        HashSet::from_iter(mem.iter().map(|x| x.iter().product()).into_iter());
    let range = prods.iter().max().unwrap() - 1;
    let avg = prods.iter().sum::<u64>() as f64 / prods.len() as f64;
    let mut prods: Vec<u64> = prods.into_iter().collect();
    prods.sort();
    let median = if prods.len() % 2 == 0 {
        (prods[prods.len() / 2] + prods[(prods.len() / 2) - 1]) as f64 / 2f64
    } else {
        prods[prods.len() / 2] as f64
    };
    format!("Range: {} Average: {:.2} Median: {:.2}", range, avg, median)
}
fn part_map(n: u64) -> String {
    let mut mem: HashMap<u64, Vec<Vec<u64>>> = HashMap::with_capacity((n) as usize);
    partitions_map(n, &mut mem);
    println!("mem: {}", mem.len());
    let mut prods: Vec<u64> = mem
        .get(&n)
        .unwrap()
        .into_iter()
        .map(|x| x.iter().product())
        .collect();
    prods.sort();
    prods.dedup();
    let range = prods.iter().max().unwrap() - 1;
    let avg = prods.iter().sum::<u64>() as f64 / prods.len() as f64;
    let median = if prods.len() % 2 == 0 {
        (prods[prods.len() / 2] + prods[(prods.len() / 2) - 1]) as f64 / 2f64
    } else {
        prods[prods.len() / 2] as f64
    };
    format!("Range: {} Average: {:.2} Median: {:.2}", range, avg, median)
}

fn partitions_set<'t>(n: u64, prefix: Vec<u64>, mem: &mut HashSet<Vec<u64>>) -> () {
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

fn partitions_map(n: u64, mem: &mut HashMap<u64, Vec<Vec<u64>>>) {
    if mem.contains_key(&n) {
        return;
    }

    let mut res = vec![vec![n]];
    for i in 1..n {
        if !mem.contains_key(&(n - i)) {
            println!("{} was not in cache", n - i);
            partitions_map(n - i, mem);
        }
        // so this works correctly. We need to get the products from this
        // So why not store the products?
        let parts = mem.get(&(n - i)).unwrap();
        for part in parts {
            let mut xs = part.clone();
            xs.push(i);
            res.push(xs);
        }
    }
    mem.insert(n, res);
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
    // test_equal(&part(1), "Range: 0 Average: 1.00 Median: 1.00");
    // test_equal(&part(2), "Range: 1 Average: 1.50 Median: 1.50");
    // test_equal(&part(3), "Range: 2 Average: 2.00 Median: 2.00");
    // test_equal(&part(4), "Range: 3 Average: 2.50 Median: 2.50");

    test_equal(&part(20), "Range: 5 Average: 3.50 Median: 3.50");
}
