// using boxes to enable recursive types
// implementing a cons list - a functional programming concept
// these consist of nested pairs of values and act like linked lists
// e.g. (1, (2, 3, (4, Nil))) is the list 1, 2, 3, 4
enum List {
    Cons(i32, Box<List>),
    Nil
}

impl List {
    pub fn print(&self) {
        match self {
            List::Cons(num, next) => {
                println!("{}", num);
                next.print();
            },
            List::Nil => {
                return;
            }
        }
    }
    pub fn add(&mut self, next_num: i32) {
        match self {
            List::Nil => {
                panic!("Should not have reached a Nil value");
            }
            List::Cons(num, next_list_item) => {
                match &**next_list_item {
                    List::Nil => {
                        *self = List::Cons(*num, Box::new(List::Cons(next_num, Box::new(List::Nil))));
                    }
                    _ => next_list_item.add(next_num),
                }
            }
        }
    }
}

fn main() {
    // storing an i32 on the heap using Box<t>
    let b = Box::new(5);
    println!("b = {b}");

    // defining a cons list
    use crate::List::{Cons, Nil};
    let mut list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    list.print();
    println!("Adding more numbers to the list");
    list.add(5);
    list.add(4);
    list.add(100);
    list.print();
}
