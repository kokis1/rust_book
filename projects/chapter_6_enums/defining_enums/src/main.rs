#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8), // data can be stored in each enum variant
    V6(String), // each variant can hold different types
}


// defining an enum in this way is less verbose and more interconnected than defining 4 separate structs
// also, each variant is of type Message, allowing them to be considered together in one function
enum Message {
    Quit,
    Mode {x: i32, y: i32}, // enums variants can have named fields
    Write(String),
    ChangeColour(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("This enum has been called!");
    }
}

fn route(ip_kind: IpAddrKind) { // enums can be passed into functions
    println!{"This IP address is of type {:?}", ip_kind}; // using the debug trait to print the type
}

fn main() {
    // defining either type 4 or type 6 IP addresses

    let four = IpAddrKind::V4(127, 0, 0, 1); // enums come with their own constructor functions
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(six);

    let message = Message::Mode {  // declaring an instance of Message
        x: 34,
        y: 0,
    };
    message.call(); // calling a method

    // Using the Option enum
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None; // None can't use type inferences, so it has to be specified manually
}
