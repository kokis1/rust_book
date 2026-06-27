use std::ops::Add;
use std::fmt;

// using associated types to perform operator overloading
#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Metres(u32);

impl Add<Metres> for Millimeters {

    // for the Add trait, its associated type is called Output
    // this is the result of the addition
    type Output = Millimeters;

    fn add (self, other: Metres) -> Millimeters {
        Millimeters(self.0 + 1000 * other.0)
    }
}

// supertraits: when the struct being implmented for must also implement from other traits

// adding a trait bound to the trait definition
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();          // the .to_string() method is guaranteed to be implemented
        let len = output.len();                 // due to the trait bound
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// in order to implement this for Millimeters or Metres, we must first implement Display

impl fmt::Display for Millimeters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Metres {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}

impl OutlinePrint for Millimeters {}
impl OutlinePrint for Metres {}

fn main() {
    let x = Millimeters(120);
    let y = Metres(1);

    x.outline_print();
    y.outline_print();

    let sum = x + y;

    println!("the sum is {}mm", sum.0);
}
