struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping custom smart pointer with data: {}", self.data);
    }
}
fn main() {
    let c = CustomSmartPointer { data: String::from("this is some data!")};
    let reuben = CustomSmartPointer { data: String::from("Reuben Stannah")};
    let message_when_dropped = CustomSmartPointer { data: String::from("I am being dropped!!")};

    // when all these variables go out of scope they are dropped, meaning that the drop function is implicitly called

    // it isn't allowed to call the drop() function manually
    // c.drop(); // this will cause a compiler error
    drop(c); // this works, it is a function that is provided by the standard library
             // this is NOT the same as calling x.drop()!

    println!("This is now the last line of code!"); // ensuring that c IS dropped early
}
