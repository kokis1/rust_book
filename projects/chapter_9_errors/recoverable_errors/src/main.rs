use std::fs::File;
use std::io::{self, ErrorKind, Read};

// Propogating errors: if a function encounters an error, it can return it to be handled by whatever called the function
fn read_username_from_file_verbose(filename: &str) -> Result<String, io::Error> {
    let username_file_result = File::open(filename);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }

}

// this achieves the same thing as the verbose version, but using the ? operator
// the ? operator is syntactic sugar that returns early if an Err(e) is found
fn read_username_from_file_shorter(filename: &str) -> Result<String, io::Error> {
    let mut username_file = File::open(filename)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// by using the ? operator in a clever way this can be shortened even further
fn read_username_from_file_shortest(filename: &str) -> Result<String, io::Error> {
    let mut username = String::new();

    // the ? operator can be chained together
    File::open(filename)?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {

    let filename: &str = "hello.txt";

    // attempt to open a file that doesn't exist - it is a result type
    let greeting_file_result: Result<File, io::Error> = File::open(filename);

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => panic!("Problem opening the file {error:?}"),
        },
    };

    // this either unwraps whatever is inside the Ok() variant, or panics if it is Err()
    // let second_greeting_file = File::open("reuben.txt").unwrap();

    // using the .expect("error message") method allows the error message to be specified
    //let third_greeting_file = File::open("third.txt").expect("No file found :( ");

    let username = match read_username_from_file_verbose(filename) {
        Ok(name) => {
            println!("The username is {name}");
            name
        },
        Err(e) => panic!("Error in finding the username: {e:?}")
    };

    let username = match read_username_from_file_shorter(filename) {
        Ok(name) => {
            println!("The username is {name}");
            name
        },
        Err(e) => panic!("Error in finding the username: {e:?}")
    };

    let username = match read_username_from_file_shortest(filename) {
        Ok(name) => {
            println!("The username is {name}");
            name
        },
        Err(e) => panic!("Error in finding the username: {e:?}")
    };

    // of course, there is inbuilt functionality for this sort of thing
    let username = match std::fs::read_to_string(filename) {
        Ok(name) => {
            println!("The username is {name}");
            name
        },
        Err(e) => panic!("Error in finding the username: {e:?}")
    };
}
