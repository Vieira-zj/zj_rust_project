use std::fmt;
use std::io;

fn greet_world() {
    let hello = "World, hello";
    println!("{}", hello);
}

fn main() {
    if false {
        greet_world();
        get_value_by_input_idx();
    }

    display_trait_sample();
    println!("done");
}

fn get_value_by_input_idx() -> () {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index:");

    let mut idx = String::new();
    io::stdin()
        .read_line(&mut idx)
        .expect("Failed to read line");

    let idx: usize = idx.trim().parse().expect("Index entered was not a number");

    let element = a[idx];
    println!("The value of the element at index {} is: {}", idx, element);
}

// trait sample

#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Close,
}

#[derive(Debug)]
struct File {
    name: String,
    _data: Vec<u8>,
    state: FileState,
}

impl fmt::Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "OPEN"),
            Self::Close => write!(f, "CLOSE"),
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    fn new(name: &str) -> File {
        Self {
            name: String::from(name),
            _data: Vec::new(),
            state: FileState::Close,
        }
    }

    fn open(&mut self) {
        self.state = FileState::Open;
    }
}

fn display_trait_sample() {
    let mut f = File::new("ftest.txt");
    println!("debug: {:?}", f);
    println!("display: {}", f);

    f.open();
    println!("after open: {}", f);
}
