mod common;

// integration test

#[test]
fn it_add_two() {
    common::setup();
    world_hello::add_two(2);
}

#[test]
fn it_var_assign_value() {
    struct TmpStruct {
        e: i32,
    }
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    TmpStruct { e, .. } = TmpStruct { e: 5 };
    println!("a={},b={},c={},d={},e={}", a, b, c, d, e);
}

#[test]
fn it_exp_and_statement() {
    let x = {
        let a = 3;
        a + 2 // exp
    };
    println!("the value of x is: {}", x);

    #[allow(unused_must_use)]
    let y = {
        let a = 3;
        a + 1; // statement
    };
    assert_eq!(y, ());

    let z = if x % 2 == 1 { "odd" } else { "even" };
    println!("z: {}", z);
}

#[test]
fn it_float_calculate() {
    use num::complex::Complex;
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
}

#[test]
fn it_loop_for_range() {
    for c in 'a'..='z' {
        print!("{},", c);
    }
    println!();

    for c in 'a'..'z' {
        print!("{},", c);
    }
    println!();
}

#[test]
fn it_return_from_fn() {
    fn plus_or_minus(x: i32) -> i32 {
        if x > 5 {
            return x - 5;
        }
        x + 5
    }

    let x = plus_or_minus(5);
    println!("the value of x is: {}", x);
}

#[test]
fn it_reference_value() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let ref z = x;
    assert_eq!(5, *z);
}

#[test]
fn it_update_string() {
    fn change(str: &mut String) {
        str.push_str(", world");
    }

    let mut s = String::from("hello");
    change(&mut s);
    println!("string: {}", s);

    s.push_str(" (test)");
    println!("string: {}", s);
}

#[test]
fn it_slice() {
    let a = [1, 2, 3, 4, 5];
    let s = &a[1..3];
    assert_eq!(s, &[2, 3])
}

#[test]
fn it_string_op() {
    let mut s = String::from("hello ");
    s.push('r');
    println!("push() -> {}", s);
    s.push_str("ust!");
    println!("push_str() -> {}\n", s);

    s.insert(5, ',');
    println!("insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("insert_str() -> {}\n", s);

    let mut string_pop = String::from("rust pop 中文");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
    println!();

    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);
    println!();

    let string_append = String::from("hello ");
    let string_rust = String::from("rust");
    // &string_rust 会自动解引用为 &str
    let result = string_append + &string_rust;
    let result = result + "!";
    println!("concatenate -> {}", result);
}

#[test]
fn it_string_convert() {
    fn first_word1(s: &String) -> &str {
        &s[..1]
    }
    fn first_word2(s: &str) -> &str {
        &s[..1]
    }

    let s = String::from("hello world");
    let word = first_word1(&s);
    println!("first word: {}", word);

    // 这里 &s 是 &String 类型，但是 first_word 函数需要的是 &str 类型
    // 尽管两个类型不一样，但是代码仍然可以工作，原因是 &String 会被隐式地转换成 &str 类型（取引用）
    let word = first_word2(&s);
    println!("first word: {}", word);
}

#[test]
fn it_debug_struct() {
    #[derive(Debug)]
    struct Rectangle {
        _width: u32,
        _height: u32,
    }

    let rect = Rectangle {
        _width: 30,
        _height: 30,
    };
    println!("rect is {:?}", rect);
    println!();

    dbg!(rect);
}

#[test]
fn it_option() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let result = plus_one(five);
    dbg!(result);

    let result = plus_one(None);
    dbg!(result);
}

#[test]
fn it_enum_match() {
    enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColorRGB(u16, u16, u16),
    }

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("say: {}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
}

#[test]
fn it_trait_object() {
    trait IPAddr {
        fn display(&self);
    }

    struct IPV4(String);
    impl IPAddr for IPV4 {
        fn display(&self) {
            println!("ipv4: {:?}", self.0)
        }
    }

    struct IPV6(String);
    impl IPAddr for IPV6 {
        fn display(&self) {
            println!("ipv6: {:?}", self.0)
        }
    }

    let ips: Vec<Box<dyn IPAddr>> = vec![
        Box::new(IPV4("127.0.0.1".to_string())),
        Box::new(IPV6("::1".to_string())),
    ];
    for ip in ips {
        ip.display();
    }
}

#[test]
fn it_int_type_convert() {
    let b: i16 = 1500;
    let b: i8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            -1
        }
    };
    println!("b: {}", b);
}

#[test]
fn it_error_handle() {
    use std::fs::File;
    use std::io;
    use std::io::Read;

    fn read_from_file() -> Result<String, io::Error> {
        let mut f = File::open("/tmp/test/log.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }

    match read_from_file() {
        Ok(s) => println!("read content:\n{}", s),
        Err(e) => println!("read error: {}", e),
    };
}
