fn main() {
    // explicitly call the panic!() macro to crash the program and print a message
    //panic!("Crash and Burn!!");

    // implicitly cause a panic by creating an unrecoverable bug
    let v = vec!(1, 2, 3);
    v[99]; // there are NOT 100 elements in v

    // call the $ RUST_BACKTRACE=1 cargo run to get a trace of exactly what caused the error
}
