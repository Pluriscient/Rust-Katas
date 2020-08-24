use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let all_letters: HashSet<char> = triplets.iter().flat_map(|x| x.iter()).cloned().collect();
    let n = all_letters.len();
    let mut message: HashMap<usize, HashSet<char>> =
        HashMap::from_iter(std::iter::repeat(all_letters).take(n).enumerate());
    let mut found_positions: HashMap<char, usize> = HashMap::new();

    for _ in 1..n * 2 {
        for letter in found_positions.keys() {
            if message.contains_key(&found_positions[letter]) {
                for (_, set) in message.iter_mut() {
                    set.remove(letter);
                }
                message.remove(&found_positions[letter]);
            }
        }
        if message.is_empty() {
            break;
        }

        for triplet in &triplets {
            let mut new_start = *message.keys().min().unwrap();
            let mut new_end = *message.keys().max().unwrap();
            if found_positions.contains_key(&triplet[0]) {
                new_start = found_positions[&triplet[0]];
            }
            if found_positions.contains_key(&triplet[2]) {
                new_end = found_positions[&triplet[2]];
            }
            // if we're not in the found positions
            // if !triplet.into_iter().any(|x| found_positions.contains_key(x)) {
            // we can then pretend to have a new outer edge
            message.entry(new_start).and_modify(|x| {
                x.remove(&triplet[1]);
                x.remove(&triplet[2]);
            });
            message.entry(new_start + 1).and_modify(|x| {
                x.remove(&triplet[2]);
            });
            message.entry(new_end - 1).and_modify(|x| {
                x.remove(&triplet[0]);
            });
            message.entry(new_end).and_modify(|x| {
                x.remove(&triplet[0]);
                x.remove(&triplet[1]);
            });
        }
        for k in message.keys() {
            if message[k].len() == 1 {
                let letter = message[k].iter().next().unwrap();
                if !found_positions.contains_key(letter) {
                    found_positions.insert(*letter, *k);
                }
            }
        }
    }
    let mut msg = vec!['-'; n];
    for (&c, &i) in found_positions.iter() {
        msg[i] = c;
    }
    println!("returning {:?}", msg);
    msg.iter().collect()
}
// Rust test example:
// TODO: replace with your own tests (TDD), these are just how-to examples.
// See: https://doc.rust-lang.org/book/testing.html

/*
taken from: yakovs83

Each triplet defines two partial order relations, so we
construct a corresponding graph with arrows going from
the 1st to the 2nd and from the 2nd to the 3rd elements
in each triplet and then start picking characters that
have no incoming arrows, removing them from the graph and
repeating the process.
*/
fn recover_secret_graph(triplets: Vec<[char; 3]>) -> String {
    type G = HashMap<char, HashSet<char>>;
    fn f(mut g: G, t: &[char; 3]) -> G {
        g.entry(t[2]).or_insert_with(HashSet::new).insert(t[1]);
        g.entry(t[1]).or_insert_with(HashSet::new).insert(t[0]);
        g.entry(t[0]).or_insert_with(HashSet::new);
        g
    }
    let mut graph = triplets.iter().fold(HashMap::new(), |acc, t| f(acc, t));
    println!("{:#?}", graph);

    let mut phrase = String::new();
    while !graph.is_empty() {
        let next = *graph.iter().find(|&(_, v)| v.is_empty()).unwrap().0;
        println!("{}", next);
        phrase.push(next);
        graph.remove(&next);
        for (_, val) in graph.iter_mut() {
            val.remove(&next);
        }
    }
    phrase
}

#[test]
fn example_test() {
    assert_eq!(
        recover_secret(vec![
            ['t', 'u', 'p'],
            ['w', 'h', 'i'],
            ['t', 's', 'u'],
            ['a', 't', 's'],
            ['h', 'a', 'p'],
            ['t', 'i', 's'],
            ['w', 'h', 's']
        ]),
        "whatisup"
    );
}
