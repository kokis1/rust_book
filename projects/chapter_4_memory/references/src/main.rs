fn main() {
    let mut s1 = String::from("hello!");
    let length = calculate_length(&s1);
    println!("The string, '{s1}', has a length of {length}");
    change(&mut s1); // a mutable reference has to be passed into the function in order to be changed
    println!("the string s1 has been changed to '{:?}'", s1);

    // let dangling = dangle(); // This can't work because a reference to empty data is created
    let no_dangling = no_dangle(); // This is allowed because there is no dangling reference created
}

fn calculate_length(s: &String) -> usize { // s is a reference to a string
    s.len()
} // Here, s goes out of scope.
// But because s does not have ownership of the value of what it refers to, the String is not dropped

fn change(s: &mut String){ // a mutable reference is passed into this function
    s.push_str(", Changed!"); // mutable references can be modified
}

/* fn dangle() -> &String {
    
    // creates a dangling reference
    let s = String::from("hello"); // a new string
    &s // returning the reference to s, however s goes out of scope: the reference is dangling!
} */

fn no_dangle() -> String {
    let s = String::from("hello");
    s // instead we can return s directly, moving ownership into the main function
}


/*
RULES FOR REFERENCES:
1: At any given time you can have EITHER:
    - ONE mutable reference
    - many immutable references
2: All references must ALWAYS be valid
*/