
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: Rectangle) -> bool {
        self.width < other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(num: i32) -> i32 {
    return num + 3; // this shows the result of a failed assert_eq!() test
    num + 2
}

pub fn greeting(name: &str) -> String {
    return String::from("hello");
    format!("Hello {name}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("This test fails!!");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 100,
            height: 50,
        };
        let smaller = Rectangle {
            width: 50,
            height: 25,
        };
        assert!(larger.can_hold(smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 100,
            height: 50,
        };
        let smaller = Rectangle {
            width: 50,
            height: 25,
        };
        assert!(!smaller.can_hold(larger));
    }

    #[test]
    fn it_adds_two() {
        let result = add_two(34);
        assert_eq!(result, 36);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was '{result}'", // the custom message is &str, passed into format!
        );
    }

    #[test]
    #[should_panic(expected = "value must be between 1 and 100")] // to make sure that the following does indeed panic
    // add an expected parameter to should_panic, containing a substring of the error message
    // this ensures the test only passes if the right panic!() is encountered
    fn greater_than_100() {
        Guess::new(200);
    }

    // tests can also return an error instead of panicking
    #[test]
    fn returns_error() -> Result<(), String> {
        let result = add(2, 4);
        match result {
            6 => Ok(()),
            _ => Err(String::from("two plus four does not equal six"))
        }
    }

    #[test]
    #[ignore] // ignores this test unless specifically requested
    fn expensive_test() {
        panic!("This is a very expensive test");
    }
}
