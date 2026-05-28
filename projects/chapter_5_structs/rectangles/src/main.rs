#[derive(Debug)] // this allows an instance of Rectangle to be printed with println!
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    let dimensions1 = (30, 50);

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "With separate variables for each dimension: The area of the rectangle is {} square metres.",
    area_vars(width1, height1)
    );

    println!(
        "Storing the dimension as a tuple: The area of the rectangle is {} square metres.",
    area_tuples(dimensions1)
    );

    println!(
        "Storing the dimensions in a struct: The area of the rectangle is {} square metres",
        area_struct(&rect1)
    );

    println!("The value of rect1 is: {rect1:?}"); // the :? after the {rect1... allows use of debug format

    let scale = 2;

    let rect2 = Rectangle { // the dbg! macro takes ownership of the expression, prints the line number and value
                            // and then returns the ownership of the expression back to where it was borrowed from
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect2);
}

fn area_vars(width: i32, height: i32) -> i32 {
    width * height
}

fn area_tuples(dimensions: (i32, i32)) -> i32 {
    dimensions.0 * dimensions.1
}
