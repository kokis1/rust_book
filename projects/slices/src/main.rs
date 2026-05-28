fn main() {
    let s = String::from("This is a string with words separated with spaces!");

    // problem: find the first word in a string of words separated by spaces

    // naive approach
    let word = naive_first_word(&s);
    println!("The index of the first word of the string is {word}");

    // better approach: using string slices
    let first_word = better_first_word(&s);
    println!("The first word of the string is {first_word}");

    // best approach: using string literals (as slices of String)
    let first_word = best_first_word(&s);
    println!("The first word of the string is {first_word}");



}

fn naive_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn better_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn best_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
