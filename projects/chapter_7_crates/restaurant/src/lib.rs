pub mod front_of_house;

fn deliver_order() {}

mod back_of_house {
    
    pub struct Breakfast {
        pub toast: String, // this is publicly accessible by other modules and crates
        seasonal_fruit: String, // this is a private field
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { // this is a public method
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup, // all fields of an enum are public
        Salad
    }


    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}
}

    
pub fn eat_at_restaurant() {
        // accessing the function using an absolute path
        // the 'crate' at the start is a literal and refers to the root crate
        crate::front_of_house::hosting::add_to_waitlist();

        // relative path
        front_of_house::hosting::add_to_waitlist();

        // ordering a breakfast in the summer of rye bread toast
        let mut meal = back_of_house::Breakfast::summer("Rye");

        // changing minds about the toast type
        meal.toast = String::from("Wheat");

        println!("I'd like to order {} toast please.", meal.toast);

        // the following won't compile because it is attempting to access a private field
        // println!("I'd like to have some {} please.", meal.seasonal_fruit);

        let appetizer1 = back_of_house::Appetizer::Soup;
        let appetizer2 = back_of_house::Appetizer::Salad;

    }

// doing the same in a new module, with the use keyword

pub mod customer {
    use crate::front_of_house::hosting;

    // using an alias to shorten path names
    use crate::back_of_house as back;

    pub fn eat_at_restaurant() {

        // with the use keyword, this is so much easier
        // also if the path changes during development, only one definition needs to be re-written
        hosting::add_to_waitlist();

        // making an order using the alias
        let order = back::Breakfast::summer("Sourdough");
    }

}


// re-exporting the use with the pub keyword to define the path or alias in other scopes

pub use front_of_house::hosting as host;

fn check_queue() {
    host::add_to_waitlist();
}

// nested paths to make the use less verbose
use front_of_house::{hosting, serving};

// the glob operator (*) is able to bring all public items in the mmodule into scope
use back_of_house::*;