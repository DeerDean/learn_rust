struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess{
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}


// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn it_not_works() {
        panic!("Make this test fail");
    }
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 20,
            height: 20,
        };
        let smaller = Rectangle {
            width: 10,
            height: 10,
        };

        assert!(larger.can_hold(&smaller));
    }

    // ---------------------
    fn add_two(a: i32) -> i32 {
        a + 2
    }

    #[test]
    fn add_right() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn add_wrong() {
        assert_ne!(3, add_two(3));
    }

    // ------------------------
    fn greeting(name: &str) -> String {
        format!("Hello!")
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }

    // ------------------------
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(999);
    }

    // -------------------------
    #[test]
    #[ignore]
    fn it_works_and_return_res() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
