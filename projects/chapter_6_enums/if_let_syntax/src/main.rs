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

impl UsStates {
    fn existed_in(&self, year: i32) -> bool {
        match self {
            UsStates::Alabama => year >= 1819,
            UsStates::Alaska => year >= 1959,
        }
    }
}

fn describe_state_quarter_naive(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin { // this uses an if let expression to get the state value
        state
    } else {
        return None; // if there is no way to match the expression, the function return the None variant of Opiton<String>
    };
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty new for America"))
    } else {
        Some(format!("{state:?} is relatively new"))
    }
}

fn describe_state_quarter_better(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else { // the let ... else {} syntax
        return None;
    };
    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty new for America"))
    } else {
        Some(format!("{state:?} is relatively new"))
    }
}

fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max { // we can bind a value in an if statement using let ... = ...
        println!("The maximum value to be considered is {max}");
         // less verbose than match because this doesn't have to be exhaustive
    }

    let penny = Coin::Penny;
    let alaskan_quarter = Coin::Quarter(UsStates::Alaska);
    let alabaman_quarter = Coin::Quarter(UsStates::Alabama);
    println!("{:?}", describe_state_quarter_better(penny));
    println!("{:?}", describe_state_quarter_naive(alaskan_quarter));
    println!("{:?}", describe_state_quarter_better(alabaman_quarter));
}
