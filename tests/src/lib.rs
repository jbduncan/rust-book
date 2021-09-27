#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

// These are "unit tests" in Rust's terminology.
// They are tests that can only test things in
// this file, and they can test both public and
// private functions and other things.
//
// The #[cfg(test)] tells cargo to not include
// the unit tests in the build artifact when
// running `cargo build`, and that they should
// be run when `cargo test1 is used.
#[cfg(test)]
mod tests {
    // This `use` allows us to import and test
    // the functions/structs in the rest of
    // this file.
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 7,
            height: 6,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 7,
            height: 6,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn rectangles_cannot_hold_themselves() {
        let rect = Rectangle {
            width: 8,
            height: 7,
        };
        assert!(!rect.can_hold(&rect));
    }

    #[test]
    fn equal_size_rectangles_cannot_hold_each_other() {
        let first = Rectangle {
            width: 8,
            height: 7,
        };
        let second = Rectangle {
            width: 8,
            height: 7,
        };
        assert!(!first.can_hold(&second));
        assert!(!second.can_hold(&first));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // An alternative to panicking, in case it's more
    // convenient e.g. you're testing something that
    // returns `Result` and want to use the `?` syntax.
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("two plus two does not equal four".to_string())
        }
    }
}
