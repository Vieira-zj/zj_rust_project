mod common;

// integration test

#[test]
fn it_add_two() {
    common::setup();
    world_hello::add_two(2);
}

//
// basic
// https://course.rs/first-try/hello-world.html
//

#[test]
fn it_var_assign_value() {
    let x = 1;
    println!("x value: {}, {}", x, &x);

    struct TmpStruct {
        e: i32,
    }
    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    TmpStruct { e, .. } = TmpStruct { e: 5 };
    println!("a={},b={},c={},d={},e={}\n", a, b, c, d, e);

    // max int value
    println!("max i8 value: {}", i8::MAX);
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
    println!("\n");

    // enumerate
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    println!();

    // break with value in loop
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;
        }
    };
    println!("result: {}", result);
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

    // ref 与 & 类似，可以用来获取一个值的引用
    let ref z = x;
    assert_eq!(5, *z);
}

//
// String, &str
//
// String 是 UTF-8 编码、可增长的动态字符串，分配在堆上，同时对于它所拥有的内容拥有所有权。
// &str 是切片引用类型（&[u8]），指向一个合法的 UTF-8 字符序列，总之，&str 和 String 的关系类似于 &[T] 和 Vec<T>.
//

#[test]
fn it_string_capacity() {
    // 事实上 String 是一个智能指针，它作为一个结构体存储在栈上，然后指向存储在堆上的字符串底层数据。
    let mut s = String::new();
    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }
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
    s += "!";
    println!("string: {}", s);

    let sub = &s[..5];
    println!("sub string: {}", sub)
}

#[test]
fn it_string_op() {
    // &str 是长度固定的字符串切片，String 是可动态增长的字符串
    // String 还是 Vector, 它们都是 Rust 的高级类型：集合类型
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
fn it_string_ref() {
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
    // 尽管两个类型不一样，但是代码仍然可以工作，原因是 &String 会被隐式地（取引用）转换成 &str 类型
    let word = first_word2(&s);
    println!("first word: {}", word);
}

#[test]
fn it_str_convert() {
    // &str => String
    let s1 = "hello, world";
    let s2 = s1.to_string();
    println!("string {}", s2);

    // String => &str
    let mut str1 = String::from("hello, world");
    let str2 = str1.as_str();
    println!("str {}", str2);

    let str3 = &mut str1;
    str3.push('!');
    println!("string {}", str3);

    fn greeting(s: &str) {
        println!("{}", s)
    }

    // Box<str> => &str
    let s: Box<str> = "hello, world".into();
    greeting(&s); // & 可以用来将 Box<str> 转换为 &str 类型
}

#[test]
fn it_str_index() {
    let s = String::from("hello, 世界");
    let s1 = &s[..1];
    assert_eq!(s1, "h");

    let s2 = &s[7..10]; // 汉字在 UTF-8 编码中占用 3 个字节
    assert_eq!(s2, "世");

    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世');
        }
    }
}

//
// array, slice
//

#[test]
fn it_array_and_slice() {
    use std::mem::size_of_val;

    let a = [1, 2, 3, 4, 5];
    let s = &a[1..3];
    assert_eq!(s, &[2, 3]);

    // 一个切片引用占用了 2 个字大小的内存空间。切片的第 1 个字是指向数据的指针，第 2 个字是切片的长度
    // 在 x86-64 上，字的大小是 64 位也就是 8 个字节，那么一个切片引用就是 16 个字节大小
    println!("size of slice: {} (bytes)", size_of_val(s));

    // 数组分配在栈上，std::mem::size_of_val 函数返回整个数组占用的内存空间
    // 数组中 char 是 unicode 字符，的每个 char 元素占用 4 字节的内存空间
    let arr: [char; 3] = ['a', 'b', 'c'];
    println!("size of array: {} (bytes)", size_of_val(&arr));
}

#[test]
fn it_display_array() {
    // 只要使用数组切片，然后传入 arr 的不可变引用，就可以打印任意长度的 i32 数组
    fn display_array(arr: &[i32]) {
        println!("values: {:?}", arr);
    }

    let arr: [i32; 3] = [1, 2, 3];
    display_array(&arr);
    let arr: [i32; 2] = [1, 2];
    display_array(&arr);

    let vect = vec![4, 5, 6];
    display_array(&vect);
}

#[test]
fn it_2d_array() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];

    for a in &arrays {
        print!("{:?}: ", a);
        for n in a.iter() {
            print!("\t{} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
}

//
// tuple, struct
//

#[test]
fn it_tuple_in_fn() {
    fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
        return (nums.0 + nums.1, nums.0 * nums.1);
    }

    let (x, y) = sum_multiply((2, 3));
    assert_eq!(x, 5);
    assert_eq!(y, 6);
}

#[test]
fn it_unit_struct() {
    // 单元结构体没有任何字段
    struct Unit;
    trait SomeTrait {}

    impl SomeTrait for Unit {}

    fn do_something_with_unit(u: Unit) {
        println!("{:p}", &u);
    }

    let u = Unit;
    do_something_with_unit(u);
}

#[test]
fn it_tuple_struct() {
    // 元组结构体
    struct Color(i32, i32, i32);

    fn check_color(c: Color) {
        let Color(x, _, _) = c;
        assert_eq!(x, 0);
        assert_eq!(c.1, 127);
        assert_eq!(c.2, 255);
    }

    let c = Color(0, 127, 255);
    check_color(c);
}

#[test]
fn it_debug_struct() {
    #[derive(Debug)]
    struct Rectangle {
        _width: u32,
        _height: u32,
    }

    // 对于结构体，我们必须为其中的每一个字段都指定具体的值
    let rect = Rectangle {
        _width: 30,
        _height: 30,
    };
    // 打印 debug 信息到标准输出 stdout
    println!("rect is {:?}", rect);
    println!();

    // 打印 debug 信息到标准错误输出 stderr
    dbg!(rect);
}

//
// enum
//

#[test]
#[allow(dead_code)]
fn it_enum_value() {
    enum Number1 {
        Zero = 0,
        One,
        Two,
    }
    enum Number2 {
        Zero = 0,
        One = 1,
        Two = 2,
    }

    // 通过 as 可以将枚举值强转为整数类型
    assert_eq!(Number1::One as u8, Number2::One as u8);
}

#[test]
#[allow(dead_code)]
fn it_enum_usage() {
    // 使用枚举对类型进行同一化
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    fn show_message(msg: Message) {
        println!("{:?}", msg)
    }

    let msgs: [Message; 3] = [
        Message::Quit,
        Message::Move { x: 1, y: 3 },
        Message::ChangeColor(255, 255, 0),
    ];
    for msg in msgs {
        show_message(msg);
    }
}

#[test]
#[allow(dead_code)]
fn it_enum_if_let() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
    }

    let msg = Message::Move { x: 1, y: 1 };
    if let Message::Move { x: a, y: b } = msg {
        assert_eq!(a, b);
    } else {
        println!("not match")
    }
}

#[test]
fn it_enum_option() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    dbg!(six);

    let none = plus_one(None);
    dbg!(none);

    if let Some(n) = six {
        println!("num: {}", n);
    }
}

//
// match
//

#[test]
fn it_match_common() {
    // num match
    let n = 7u8;
    match n {
        1 => println!("match 1"),
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        6..=10 => println!("match 6 -> 10"),
        _ => {
            println!("match 11 -> +infinite");
        }
    }

    // enum match
    #[allow(dead_code)]
    enum Direction {
        East,
        West,
        North,
        South,
    }

    let direction = Direction::South;
    match direction {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        }
        _ => println!("West"),
    }
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
fn it_enum_match_with_cond() {
    // match by if
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // match by range
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 11 };
    match msg {
        Message::Hello { id: newid @ 3..=7 } => {
            println!("Found an id in range: {}", newid);
        }
        Message::Hello { id: id @ 10..=12 } => {
            println!("Found an id in another range: {}", id);
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id);
        }
    }
}

#[test]
fn it_enum_matches() {
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    let bar = Some(4);
    assert!(matches!(bar, Some(x) if x > 2));

    #[derive(Debug)]
    enum MyEnum {
        Foo,
        Bar,
    }
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
    let iter: Vec<&MyEnum> = v.iter().filter(|x| matches!(x, MyEnum::Foo)).collect();
    println!("{:?}", iter);
}

#[test]
fn it_match_in_while_let() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("value: {}", top);
    }
}

//
// 类型转换
//

#[test]
fn it_num_conv_by_as() {
    // Rust 并没有为基本类型提供隐式的类型转换，可以通过 as 来进行显式转换
    let a = 3.1 as i8;
    let b = 100_i8 as i32;
    let c = 'a' as u8;
    println!("{},{},{}", a, b, c);
}

#[test]
fn it_str_conv_to_string() {
    // String 实现了 From<&str> 特征
    let my_str = "hello";
    let string1 = String::from(my_str);
    let string2 = my_str.to_string();
    // 这里需要显式地类型标注
    let string3: String = my_str.into();
    println!("{},{},{}", string1, string2, string3);
}

#[test]
fn it_num_conv_by_tryinto() {
    // 与 From/Into 不同，TryFrom 和 TryInto 可以对转换后的失败进行处理，然后返回一个 Result
    use std::convert::TryInto;

    let b: i16 = 1500;
    let b: i8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("error: {:?}", e.to_string());
            -1
        }
    };
    println!("b: {}", b);
}

#[test]
fn it_from_and_into_trait() {
    // From 特征 let u: U = U::from(T)
    // Into 特征 let u:U = T.into()
    #[derive(Debug, PartialEq)]
    struct EvenNum(i32);

    impl TryFrom<i32> for EvenNum {
        type Error = ();
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNum(value))
            } else {
                Err(())
            }
        }
    }

    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));

    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8)));
    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

#[test]
fn it_pointer_addr_conv() {
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize;
    // i32 类型占用 4 个字节
    println!("i32 size: {} (bytes)", std::mem::size_of::<i32>());
    let second_address = first_address + 4;
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values[1], 3);
}
