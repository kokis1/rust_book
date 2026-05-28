#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool { // a method can share a name with a field
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // this is an associated function, one defined in the impl block
    // associated functions don't take as an argument a self, so don't require an instance of Self to exist
    // these are used for constructors among other things
    fn square(size: u32) -> Self { // this creates an instance of Rectangle that is a square
        Self {
            width: size,
            height: size
        }
    }
}

impl Rectangle { // one struct can have multiple impl blocks
    fn rectangle(width: u32, height: u32) -> Self {
        Self {
            width,
            height
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 20,
        height: 60
    };

    println!(
        "The area of the rectangle rect1 is {} square pixels.",
        rect1.area()
    );

    println!(
        "The fact that width is greater than 0 is: {}",
        rect1.width()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square1 = Rectangle::square(30); // to call an associated function use :: after the namespace identifier
    println!("The square is: {square1:?}");

    let rect4 = Rectangle::rectangle(30, 40);
    println!("rect4 is: {rect4:?}");

}