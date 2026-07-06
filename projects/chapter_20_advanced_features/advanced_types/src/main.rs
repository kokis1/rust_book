fn main() {
    
    // adding a type synonym to help with readibility
    type Kilometers = i32;

    let x: i32 = 56;
    let y: Kilometers = 123;

    // y is still sort of an i32, just with an alias
    // this is normally used in long implementations with lots of repitition of the same complicated types
    println!("{}", x + y);
    

    // using the never type (!): it's special property is that it can be coerced into any other type
    let stupid_unwrap = |x: Option<i32>| {
        match x {
            Some(x) => x,
            None => panic!(), // panic!() has type of !, so the whole closure is coerced into type of i32
        }
    };

    println!("{}", stupid_unwrap(Some(4)));
}



// from type theory's Empty Type: the Never Type, denoted as !
// fn bar() -> ! { (this won't actually compile btw)
    // this function will never return anything
//}
