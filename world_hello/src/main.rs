use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::Read;

fn greet_world() {
    let hello = "World, hello";
    println!("{}", hello);
}

fn main() {
    unsafe {
        CONFIG = init_config();
        println!("{:?}", CONFIG);
    }

    if false {
        greet_world();
        word_count();

        let word = first_word("hello world");
        println!("first word: {}", word);

        string_parse();
        get_value_by_input_index();

        read_file_sample_01();
        read_file_sample_02();

        display_trait_sample();

        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        retain_even(&mut v);
        println!("even num: {:?}", v);
    }

    custom_macro_sample();
    println!("done");
}

// common samples

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn word_count() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1
    }
    println!("word count: {:?}", map);
}

fn string_parse() {
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

fn get_value_by_input_index() -> () {
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

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn retain_even(nums: &mut Vec<i32>) {
    let mut i = 0;
    for j in 0..nums.len() {
        if is_even(nums[j]) {
            nums[i] = nums[j];
            i += 1;
        }
    }
    nums.truncate(i);
}

// read file samples

fn read_file_sample_01() {
    let path = "/tmp/test/log.txt";
    match read_from_file(path) {
        Ok(s) => println!("read file:\n{}", s),
        Err(e) => println!("read {} error: {}", path, e),
    };
}

fn read_from_file(path: &str) -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_file_sample_02() {
    let path = "/tmp/test/log.txt";
    match std::fs::read_to_string(path) {
        Ok(s) => println!("read file:\n{}", s),
        Err(e) => println!("read {} error: {}", path, e),
    };
}

// trait sample

#[derive(Debug, PartialEq)]
enum TestFileState {
    Open,
    Close,
}

#[derive(Debug)]
struct TestFile {
    name: String,
    _data: Vec<u8>,
    state: TestFileState,
}

impl fmt::Display for TestFileState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "OPEN"),
            Self::Close => write!(f, "CLOSE"),
        }
    }
}

impl fmt::Display for TestFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl TestFile {
    fn new(name: &str) -> TestFile {
        Self {
            name: String::from(name),
            _data: Vec::new(),
            state: TestFileState::Close,
        }
    }

    fn open(&mut self) {
        self.state = TestFileState::Open;
    }
}

fn display_trait_sample() {
    let mut f = TestFile::new("ftest.txt");
    println!("debug: {:?}", f);
    println!("display: {}", f);

    f.open();
    println!("after open: {}", f);
}

// global config

#[allow(dead_code)]
#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}

static mut CONFIG: Option<&mut Config> = None;

fn init_config() -> Option<&'static mut Config> {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });
    // 将 c 从内存中泄漏，变成 'static 生命周期
    Some(Box::leak(c))
}

// 自定义 derive 过程宏

fn custom_macro_sample() {
    use hello_macro_derive::HelloMacro;
    use world_hello::HelloMacro;

    #[derive(HelloMacro)]
    struct Sunfei;

    #[derive(HelloMacro)]
    struct Sunface;

    Sunfei::hello_macro();
    Sunface::hello_macro();
}
