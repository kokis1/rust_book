use std::ops::Deref;

// Defining a new smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// implementing the Deref trait for MyBox
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// using Deref coercion
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, 5);
    assert_eq!(5, *y);

    // using Deref coercion
    let m = MyBox::new(String::from("Reuben"));

    // since MyBox implements Deref, calling &MyBox<String> will coerce into &String
    // since &String return &str, this can then be used as the argument for hello()
    hello(&m);

    // without coercion the whole thing would have to look like this
    hello(&(*m)[..]); // dereferencing &MyBox<String> into String
                      // and then turning it into a slice over all elements
    
}
