// using function pointers to pass functions as arguments (with the fn type)
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}


// returning a closure from a function
// not possible directly, closures aren't types, but implementations of the Fn, FnOnce and FnMut traits.
// but you can return a generic that implements these
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}


fn main() {

    // passing the function add_one as an argument
    println!("{}", do_twice(add_one, 4));

    // using the closure returned from the function
    println!("{}", returns_closure()(4));
}
