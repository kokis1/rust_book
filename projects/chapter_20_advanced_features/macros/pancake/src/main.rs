use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;



#[derive(HelloMacro)]
struct Pancake {
    name: String,
    n_ingredients: u32,
}


fn main() {
    Pancake::hello_macro();

}
