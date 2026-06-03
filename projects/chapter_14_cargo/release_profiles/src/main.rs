//! # My-Crate
//! This style of documentation comment will produce markdown documentation for the item _containing_ the comments
//! this will all still show up when the command $ cargo doc is run





/// Checks whether a number is prime or not
/// 
/// These are documentation comments (3*/) that can output documentation in markdown.
/// 
/// build the documentation for this crate by running $ cargo doc <br>
/// see the built documentation by running $ cargo doc --open
///
/// ## Commonly used sections
/// 
/// # Examples
/// ```
/// let x = 100;
/// let primality = is_prime(x);
/// assert_eq!(false, primality);
/// ```
/// # Panics
/// ```
/// let x = 3.4;
/// let primality = is_prime(x);
/// ```
/// # Errors
/// The reasons that cause this to return a Result enum
/// 
/// # Safety
/// If the function is unsafe to call there should be a reason for why this is so <br>
/// This section should, if it is unsafe, describe all the invariants that the programmer must uphold.
/// 
/// # Note About Documentation Comments:
/// they can be used as tests, any code in examples (embedded within the markdown) will be run with $ cargo test
fn is_prime(n: i32) -> bool {
    for i in 1..n.isqrt() {
        if i % 1 == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("this is expensive code that needs to be optimised");

    println!("is 1234 prime? {}", is_prime(1234));
}
