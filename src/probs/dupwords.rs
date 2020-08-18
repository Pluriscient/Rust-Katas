fn remove_duplicate_words(s: &str) -> String {
    let words = s.split_whitespace().fold(vec![], |mut w, s| {
        if !w.contains(&s) {
            w.push(s);
        }
        w
    });
    words.join(" ")
    // format!("{}", s) // you code here
}
// Rust test example:
#[test]
fn sample_test_cases() {
    assert_eq!(
        remove_duplicate_words(
            "alpha beta beta gamma gamma gamma delta alpha beta beta gamma gamma gamma delta"
        ),
        "alpha beta gamma delta"
    );
    assert_eq!(
        remove_duplicate_words("my cat is my cat fat"),
        "my cat is fat"
    );
}
