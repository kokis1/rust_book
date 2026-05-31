// generics in function signatures
// The type name T is specified in the <..> after the function name, with the trait that T must also implement
fn largest_list_item<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// generics in struct definitions

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

// this is how to implement methods for a generic struct
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// how to implement a method for a struct of a specific type
impl Point<f32> { // don't need to declare any type after impl
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Point_flex<T, U> {
    x: T,
    y: U,
}

impl<X1, Y1> Point_flex<X1, Y1> {
    // mixup<X2, Y2> is needed because they haven't already been declared
    fn mixup<X2, Y2>(self, other: Point_flex<X2, Y2>) -> Point_flex<X1, Y2> {
        Point_flex {
            x: self.x,
            y: other.y
        }
    }
}

// in enums it's the exact same thing
enum Output<T, U> {
    WebServer(T, U),
    Local(U), // or something
}


fn main() {
    let list = vec![1, 2, 100, 34, 5];
    let largest = largest_list_item(&list);

    println!("the largest item in the list is {largest}");

    let point1 = Point {x: 3, y: 6};
    let point2 = Point {x: 1.4, y: 5.6};
    println!("point1 is {point1:?}; point2 is {point2:?}");

    println!("The distance of point2 from the origin is {}", point2.distance_from_origin());

    // because the only type in the definition is T, both x and y have to have the same type
    // let wont_work = Point {x: 34, y: '5'};

    // the more flexible one will allow two different types
    let will_work = Point_flex {x: 34, y: vec![1, 3, 4]};
    let will_work2 = Point_flex {x: 100, y: "hello"};
    println!("The point is {will_work:#?}");
    println!("The two points mixed up are {:#?}", will_work.mixup(will_work2));

}
