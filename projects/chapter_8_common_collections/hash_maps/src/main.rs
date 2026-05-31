// they aren't automatically in scope
use std::collections::HashMap;

fn main() {
    // making a new, mutable HashMap
    let mut scores = HashMap::new();

    // there is no built-in macro to construct these like with String or Vec
    // all key-value pairs have to have the same pair of types
    scores.insert(String::from("Team Blue"), 10);
    scores.insert(String::from("Team Red"), 15);

    
    // access elements with the .get() method
    let team_blue_score = match scores.get(&String::from("Team Blue")) {
        Some(i) => i,
        None => &0,
    };
    println!("Team Blue has a score of {team_blue_score}");
    
    // we can iterate over each pair of key and value using a tuple
    for (key, value) in &scores {
        println!("{key} has a score of {value}");
    }

    // how to insert a kay in the case that the key is already assigned

    scores.insert(String::from("Team Red"), 25); // overwriting the previous value
    println!("{scores:?}");
    
    scores.entry(String::from("Team Blue")).or_insert(50); // overwrites only if the entry doesn't exist already
    println!("{scores:?}");

    // inserting a key, using the previous value
    let mut map = HashMap::new();
    let text = "hello world wonderful world";

    // iterating through each word in text
    for word in text.split_whitespace() {

        // getting a mutable reference to the value at the key of each word
        let count: &mut i32 = map.entry(word).or_insert(0);

        // incrementing the count using the dereferencing operator
        *count += 1;
    }
    println!("{:?}", map);
    
}
