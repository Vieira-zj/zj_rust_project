pub mod tutorial;

// fn for integration test

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// fn for unit test

#[allow(dead_code)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[allow(dead_code)]
fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[allow(dead_code)]
struct Guess {
    _value: i32,
}

#[allow(dead_code)]
impl Guess {
    pub fn new(value: i32) -> Guess {
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
        Guess { _value: value }
    }
}

// unit test

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 6)
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // failed test case
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Sunface");
        let expect = "sunface";
        assert!(
            result.contains(expect),
            "result={}, expect={}",
            result,
            expect
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[ignore]
    fn expensive_test() {}
}
