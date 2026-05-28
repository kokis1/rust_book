fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");
    takes_ownership(s2); //s2 is now out of scope, the function has now got ownership

    let x = 5;
     
    makes_copy(x); //x is still in scope because x implements the Copy trait,
                   // so it's value is copied into the function

    let s3 = gives_ownership();

    let s4: String = String::from("Woohoo!");

    let s5 = takes_and_gives_back(s4);


}

fn takes_ownership(some_string: String) {
    println!("Ha-ha! I've stolen ownership of {some_string}");
}

fn makes_copy(some_integer: i32) {
    println!{"I have made a copy of {some_integer}"};
}

fn gives_ownership() -> String {
    let some_string = String::from("Yours!");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    println!("I've borrowed the string: {some_string}");
    some_string
}
