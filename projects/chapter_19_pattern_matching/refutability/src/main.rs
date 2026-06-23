fn main() {

    // irrefutable patterns are those which match any expression
    // e.g. x in:
    let x = 5;

    // e.g. y, z in:
    let (y, z) = (34, 10);

    // let expressions and for loops can only use these
    let some_option_value = None;

    // this wouldn't work because None is the true value
    // so the compiler says this isn't ever allowed, even if it would make sense
    // let Some(a) = some_option_value;

    println!("Hello, world!");
}
