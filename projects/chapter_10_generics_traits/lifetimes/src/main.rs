use std::fmt::Display;

// writing a function that returns the longest of 2 string slices

// 'a means that each parameter must have a lifetime at least as long as 'a
// in practice this means that the shorter of the two lifetimes passed into longest() is returned
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {x} else {y}
}

// lifetime syntax:
// &i32: a reference
// &'a i32: a reference with an explicit lifetime
// &'a mut i32: a mutable reference with an explicit lifetime


// using lifetimes in struct syntax
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// using lifetimes for impl
impl<'a> ImportantExcerpt<'a> {
    fn level(&self, announcement: &str) -> &str{
        println!("Attention Please! {announcement}");
        self.part
    }
}

// using lifetimes, generics and trait bounds all in one go!
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display, // T can be anything as long as it can be printed
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("the longest string is {result:?}");

    // defining a struct to hold a reference
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let part = i.level("This is a Test!");
    println!("{part}");

    // all str are static, meaning they live for the duration of the program
    let immortal_string: &'static str = "I will never die!!";

    println!("{immortal_string}");


    let longest2 = longest_with_an_announcement(string1.as_str(), string2, "Announcement!");
    println!("{longest2}");
}
