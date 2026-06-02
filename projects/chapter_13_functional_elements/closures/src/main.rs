use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColour {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColour>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColour>) -> ShirtColour {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self) -> ShirtColour {
        let mut num_red = 0;
        let mut num_blue = 0;

        for colour in &self.shirts {
            match colour {
                ShirtColour::Red => num_red += 1,
                ShirtColour::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            return ShirtColour::Red;
        } else {
            return ShirtColour::Blue;
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColour::Blue, ShirtColour::Red, ShirtColour::Blue, ShirtColour::Blue]
    };

    let user_pref1 = Some(ShirtColour::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );


    // defining a closure with no type annotations will force the compiler to infer type from the first
    // usage of it

    let simple_example = |x| x;

    // since simple_example() is not type annotated, rustc infers x is a String
    let s = simple_example(String::from("Hello, world!")); // first usage
    // let n = simple_example(5); // invalid second usage because the type has been inferred to be String

    // closures can decide whether they need to borrow, referenence immutably or reference mutably based on context
    let mut list = vec![1, 3, 5, 6];

    // only an immutable borrow is needed to print the list
    let print_list = || println!("This is the list {list:#?}"); // note the use of variables external to the closure being used
                                                            // list is defined in the same scope, so it can be used with
                                                            // no arguments
    print_list();
    println!("I still have ownership! (authored by {list:?})");

    // an example of a closure that takes a mutable referenve
    let mut borrows_mutably = |x| list.push(x);

    // println!("Unable to print {list:?} :("); // must wait until after the mutable borrow has finished

    borrows_mutably(4);
    println!("After being altered: {list:?}");


    let mut list = [
        Rectangle {width: 10, height: 1},
        Rectangle {width: 3, height: 5},
        Rectangle {width: 7, height: 12},
    ];

    list.sort_by_key(|r| r.width);
    println!("A list of Rectangles after being sorted by width:\n{list:#?}");


}
