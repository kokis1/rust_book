enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates),
}

#[derive(Debug)]
enum UsStates {
    Alabama,
    Alaska,
    // etc...
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { // match expressions don't need to evaluate to a bool like they do for if
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State Quarter from {state:?}");
            25
        }, // the match expression binds to the value in the enum
    }
}

// using the Option<T? match pattern
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // All cases need to be considered in a match expression
        Some(i) => {
            println!("Adding 1 to {i}");
            Some(i+1)
        },
    }
}

fn add_fancy_hat() {
    println!("Adding fancy hat to player!");
}
fn remove_fancy_hat() {
    println!("Removing fancy hat from player");
}
fn move_player(num_steps: i32) {
    println!("Moving player {num_steps} steps");
}
fn re_roll() {
    println!("You have to re-roll!");
}
fn turn(dice_roll: i32) {
    match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other), // the catch-all pattern consideres all other cases
        }
}
fn turn_2(dice_roll: i32) {
    match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => re_roll(), // the catch-all pattern that does not bind to a value
        }
}

fn turn_3(dice_roll: i32) {
    match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => (), // we don't want to do anything in the default case, so return the empty tuple
        }
}

fn main() {
    let penny = Coin::Penny;
    let quarter = Coin::Quarter(UsStates::Alabama);
    println!("The value of penny in cents is {}", value_in_cents(penny));
    println!("The value of quarter in cents is {}", value_in_cents(quarter));

    let some_x: Option<i32> = Some(34);
    let none_x: Option<i32> = None;

    plus_one(some_x);
    plus_one(none_x);

    // using the default value
    let dice_roll = 9;
    turn(dice_roll);
    let dice_roll = 7;
    turn(dice_roll);
    let dice_roll = 3;
    turn(dice_roll);

    // using the default value in the match expression
    let dice_roll = 4;
    turn_2(dice_roll);

    let dice_roll = 5;
    turn_3(dice_roll);

}
