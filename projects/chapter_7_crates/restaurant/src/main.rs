// using modules from other files in the main.rs that will be compiled into the crate's binary

use restaurant::front_of_house::hosting as host;
use restaurant::customer;

fn main() {
   host::add_to_waitlist();
   customer::eat_at_restaurant();
}