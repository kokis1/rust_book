fn main() {
    let number = 5;

    if number % 4 == 0 {
        println!("The number {number} is divisible by 4");
    } else if number % 3 == 0{
        println!("The number {number} is divisible by 3");
    } else if number % 2 == 0 {
        println!("The number {number} is divisible by 2");
    } else {
        println!("The number {number} is not divisible by 4, 3, or 2");
    }

    let condition = true;

    let number = if condition {5} else {6};
    println!("The value of number is: {number}");


    let mut counter = 0;
    let result = 'loop_label: loop {
        println!("Again!");
        counter += 1;
        if counter == 10 {
            break 'loop_label counter * 2;
        }
    };
    println!("The result is: {result}");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for elem in a {
        println!("The value is: {elem}");
    }
}
