use std::fmt;

// common samples

pub fn greet_world(is_run: bool) {
    if is_run {
        let hello = "World, hello";
        println!("{}", hello);
    }
}

pub fn word_count(is_run: bool) {
    if !is_run {
        return;
    }

    use std::collections::HashMap;

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }
    println!("word count: {:?}", map);
}

pub fn first_word(s: &str, is_run: bool) {
    if !is_run {
        return;
    }

    let mut word = &s[..];
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            word = &s[0..i];
        }
    }
    println!("first word: {}", word);
}

pub fn text_parse(is_run: bool) {
    if !is_run {
        return;
    }

    let penguin_data = "\
   common name,length (cm)
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,data
   ";

    let records = penguin_data.lines();
    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(",").map(|field| field.trim()).collect();
        if cfg!(debug_assertions) {
            // 只在 debug 模式下生效
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}

pub fn get_value_by_input_index(is_run: bool) -> () {
    if !is_run {
        return;
    }

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index:");

    let mut idx = String::new();
    std::io::stdin()
        .read_line(&mut idx)
        .expect("Failed to read line");

    let idx: usize = idx.trim().parse().expect("Index entered was not a number");

    let element = a[idx];
    println!("The value of the element at index {} is: {}", idx, element);
}

pub fn retain_even_numbers(nums: &mut Vec<i32>) {
    let mut i = 0;
    for j in 0..nums.len() {
        if is_even(nums[j]) {
            nums[i] = nums[j];
            i += 1;
        }
    }
    nums.truncate(i);
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

// trait samples

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Close,
}

impl fmt::Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "OPEN"),
            Self::Close => write!(f, "CLOSE"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Close,
        }
    }

    fn open(&mut self) {
        self.state = FileState::Open;
    }
}

pub fn display_trait_sample(is_run: bool) {
    if is_run {
        let mut f = File::new("ftest.txt");
        println!("file info (debug): {:?}", f);
        println!("file info: {}", f);

        f.open();
        println!("opened file info: {}", f);
    }
}

// unit test samples

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
