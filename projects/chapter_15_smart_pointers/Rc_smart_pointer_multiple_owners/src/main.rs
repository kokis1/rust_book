enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc; // Rc smartpointers need to be explicitly brought into scope

fn main() {
    let a = Rc::new(Cons(1, Rc::new(Cons(3, Rc::new(Nil)))));
    println!("Count after creating a: {}", Rc::strong_count(&a)); // counting the number of references to ad
    let b = Cons(1, Rc::clone(&a)); // these duplicate the references to ensure borrowing rules are respected
    println!("Count after creating b: {}", Rc::strong_count(&a));
    let c = Cons(5, Rc::clone(&a)); // the smart pointer will only go out of scope when it is known nothing points to it
    println!("Count after creating c: {}", Rc::strong_count(&a));
}
