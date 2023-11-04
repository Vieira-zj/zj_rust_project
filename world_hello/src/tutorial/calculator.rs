use std::io;

pub fn run() {
    println!("Enter the 1st number:");
    let x = input_parser();
    if f64::is_nan(x) {
        println!("Invalid input");
        return;
    }

    println!("Enter the 2nd number:");
    let y = input_parser();
    if f64::is_nan(y) {
        println!("Invalid input");
        return;
    }

    println!("List of operators:");
    println!("(1) Add");
    println!("(2) Subtract");
    println!("(3) Multiply");
    println!("(4) Divide");
    println!("Select the number associated with the desired operation:");

    let op = input_parser();
    if f64::is_nan(op) {
        println!("Invalid input");
        return;
    }

    let result: f64;
    let op = op as i32;
    match op {
        1 => result = x + y,
        2 => result = x - y,
        3 => result = x * y,
        4 => result = x / y,
        _ => {
            println!("Invalid selection");
            return;
        }
    }

    println!("The result is: {}", result)
}

fn input_parser() -> f64 {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Invalid input");
    let x: f64 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return f64::NAN;
        }
    };
    x
}
