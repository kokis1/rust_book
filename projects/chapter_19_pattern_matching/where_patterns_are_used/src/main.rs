// function parameters can also be patterns!
fn example(&(x, y): &(i32, i32)) {
    println!("{x}, {y}");
}




fn main() {
    /*
    a match expression uses pattern matching in this way:
    match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    }

    they must be exhaustive
     */

    let x: Option<i32> = Some(4);

    match x {
        Some(i) => println!("{}", i + 1),
        None => println!("None!!"),
    };

    // Technically, any let expression is a pattern matching action
    
//  let PATTERN = VALUE
    let y       = format!("Hello, world!");

    // using patterns to destruct a tuple
    let (a, b, c) = (1, 3, 4);

    println!("{a}, {b}, {c}");

    /* while loops continue for as long as the pattern matches
        while PATTERN {
            [...]
        } */
    
    while let Some(z) = x {
        if z > 1 {
            break;
       }
    }

    example(&(3, 56));
}
