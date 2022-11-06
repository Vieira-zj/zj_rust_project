use std::io;

fn greet_world() {
    let hello = "World, hello";
    println!("{}", hello);
}

fn main() {
    if false {
        greet_world();
    }
    get_value_by_input_idx();
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
