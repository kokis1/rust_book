fn main() {
    // creating a new, empty mutable string
    let mut s = String::new();

    // this is a string literal, of type str (can be borrowed as a string slice: &str[..])
    let data = "This is a string";

    // data can be turned into a String by giving it ownership to s (shadowing the definition of s)
    let s = data.to_string();

    // the same method works directly on str literals
    let s = "This is a string".to_string();

    // obviously, we can use the String::from() associated function to make a new, non-empty string
    let s = String::from("This is a String");

    // Updating strings:

    // pushing onto a string (because String is a wrapper for Vec)
    let mut s1 = String::from("foo ");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");

    // the .push("s") method pushes a single character to the end of the string
    s1.push('R');
    println!("s1 is {s1}");

    // concatenating Strings with the + operator
    // it takes this form because of the way the add() function is generated
    // the add function has signature of something like: fn add(self, &s) -> String {...}
    let s3 = s1 + &s2;
    println!("{s3}");

    // concatenating Strings with the format!() macro:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    // format!() doesn't change the ownership so s1, s2 and s3 are till valid
    println!("{s1}, {s2}, {s3}");

    // String indexing isn't trivial because UTF-8 characters take up 2 bytes of storage
    // Hence indexing wouldn't correspond to the desired character
    // However, you can use string slices
    let hello: &str = "Здравствуйте";
    let s: &str  = &hello[0..4];
    println!("The slice of the str hello is {s}");
    // > The slice of the str hello is Зд (note how 4 bytes becomes 2 Cyrillic letters)


    // there are two ways to get the individual elements of strings, iterating over either the chars or bytes
    let word = "Зд".to_string();

    for c in word.chars() {
        println!("{c}"); // prints normal looking letters
    }

    for b in word.bytes() {
        println!("{b}"); // prints the byte value stored in the String
    }

}
