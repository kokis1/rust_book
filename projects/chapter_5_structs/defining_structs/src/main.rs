struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,

        // this uses field-init shorthand, where the field names are the same as the function arguments
        // therefore it is not needed to type username: username to set that field
        username,
        email,
        sign_in_count: 1
    }
}

 // tuple structs - named tuples for which an entire struct with named fields is verbbose
struct Colour(u32, u32, u32);
struct Point(f32, f32, f32);

// unit-like struct: a struct with no data, useful for implementing behaviours for a named type
struct AlwaysEqual;

fn main() {
    let mut user = User { // the instance must be mutable in order to change the fields
        active: true,
        username: String::from("reubenstannah"),
        email: String::from("reuben.stannah@table.com"),
        sign_in_count: 1,
    };
    user.email = String::from("second.email@table.com");

    let mut user2 = build_user(String::from("reuben.email@address.com"), String::from("kokis"));
    println!("username: {}, email address: {}", user2.username, user2.email);

    let user3 = User { // using the update struct syntax to shorten a struct definition based on a previously defined one
        username: String::from("second-example-username"),
        ..user2
    };

    println!("username: {}, email address: {}", user3.username, user3.email);

    // defining and accessing fields of tuple structs
    let black = Colour(0, 0, 0);
    let origin = Point(1.2, 3.5, 1.0);

    let subject = AlwaysEqual;

}
