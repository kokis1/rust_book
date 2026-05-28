fn main() {
    println!("Hello, world!");

    let x = five();

    another_function(x);
    print_labelled_measurement(45, 'h');

    let x = plus_one(x);

    println!("Five {x}");
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labelled_measurement(value: i32, unit_measurement: char) {
    println!("The measurement is {value}{unit_measurement}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}