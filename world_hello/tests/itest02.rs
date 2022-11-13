use world_hello;
use world_hello::tutorial;

//
// fn of src/ lib
//

#[test]
fn it_add_point() {
    use tutorial::add_point;
    use tutorial::Point;

    let p1 = Point {
        x: 1.1f32,
        y: 1.1f32,
    };
    let p2 = Point {
        x: 2.1f32,
        y: 2.1f32,
    };
    println!("{:?}", add_point(p1, p2));

    let p3 = Point { x: 1i32, y: 1i32 };
    let p4 = Point { x: 2i32, y: 2i32 };
    println!("{:?}", add_point(p3, p4));
}

//
// format, print
//

#[test]
fn it_print_common() {
    let s = "hello";
    let s1 = format!("{}, world", s);
    println!("{}", s1);

    println!("{a},{b},{c}", a = 'a', b = "b", c = 10);
    println!();

    let v = 3.1415926;
    println!("{:.2}", v);
    println!("{:.0}", v); // 不带小数

    let s = "hello";
    println!("{:.3}", s); // 保留字符串前 3 个字符
    println!();

    let x = 5;
    let y = &x;
    println!("address of x: {:p}", y); // 指针地址
    println!();

    // 字符串右填充空格
    println!("{:5}!", "x");
    // 数字左填充空格
    println!("{:5}!", 1);
    // 数字左填充 0
    println!("{:05}!", 1);
    println!();

    println!("{:#b}!", 27); // 二进制 => 0b11011
    println!("{:#o}!", 27); // 八进制 => 0o33
    println!("{:#x}!", 27); // 小写十六进制 => 0x1b
    println!("{:#X}!", 27); // 大写十六进制 => 0x1B
}

#[test]
fn it_debug_trait() {
    // {:?} 适用于实现了 std::fmt::Debug 特征的类型
    #[derive(Debug)]
    struct Person {
        _name: String,
        _age: u8,
    }

    let s = String::from("hello");
    let v = vec![1, 2, 3];
    let p = Person {
        _name: "foo".to_string(),
        _age: 18,
    };
    println!("debug: {:?}, {:?}, {:?}", s, v, p);
}

#[test]
fn it_display_trait() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8,
    }

    // {} 适用于实现了 std::fmt::Display 特征的类型
    use std::fmt;
    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "name={},age={}", self.name, self.age)
        }
    }

    let p = Person {
        name: "bar".to_string(),
        age: 31,
    };
    println!("display: {}", p);
}

#[test]
fn it_extern_display_trait() {
    // 为外部类型实现 Display 特征
    struct Array(Vec<i32>);

    use std::fmt;
    impl fmt::Display for Array {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "array: {:?}", self.0)
        }
    }

    let arr = Array(vec![1, 2, 3]);
    println!("{}", arr)
}

//
// method
//

#[test]
fn it_method_declare() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // 关联函数
        // Self = Rectangle
        fn new(width: u32, height: u32) -> Self {
            Rectangle {
                width: width,
                height: height,
            }
        }
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn width(&self) -> u32 {
            return self.width;
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
        fn translate(&mut self, width: u32, height: u32) {
            self.width += width;
            self.height += height;
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "the area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let mut rect2 = Rectangle::new(20, 40);
    rect2.translate(20, 20);
    println!("rectangle width: {}", rect2.width());
    println!("rect2 can hold rect1: {}", rect2.can_hold(&rect1));
}

//
// generic
//

#[test]
fn it_generic_fn_declare_01() {
    // #1
    fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
        x + y
    }

    let result = sum(2i8, 10i8);
    println!("sum result: {}", result);
    let result = sum::<i16>(20, 30);
    println!("sum result: {}", result);
    println!();

    // #2
    fn display_array<T: std::fmt::Debug>(arr: &[T]) {
        println!("values: {:?}", arr);
    }

    let arr: [i32; 3] = [1, 2, 3];
    display_array::<i32>(&arr);
    let arr: [i32; 2] = [1, 2];
    display_array(&arr);

    let vect = vec!['a', 'b', 'c'];
    display_array(&vect);
}

#[test]
fn it_generic_fn_declare_02() {
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        let mut largetst = &list[0];
        for item in list.iter() {
            if item > largetst {
                largetst = item;
            }
        }
        largetst
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest::<i32>(&number_list);
    println!("the largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest::<char>(&char_list);
    println!("the largest char is {}", result);
}

#[test]
fn it_generic_struct_declare() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 1, y: 2 };
    println!("x={}, y={}", p.x(), p.y);

    let p = Point::<f32> { x: 1.0, y: 2.0 };
    println!("distance: {:.2}", p.distance_from_origin());
}

#[test]
fn it_generic_for_const() {
    fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("{:?}", arr)
    }

    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);
    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}

//
// trait
//

#[test]
fn it_trait_declare() {
    trait Summary {
        // 默认实现
        fn summarize(&self) -> String {
            String::from("(read more ...)")
        }
    }

    struct Post {
        title: String,
        author: String,
        content: String,
    }
    impl Summary for Post {
        fn summarize(&self) -> String {
            format!(
                "title:{}, author:{}, content:{}",
                self.title, self.author, self.content
            )
        }
    }

    struct Weibo {
        _username: String,
        _content: String,
    }
    impl Summary for Weibo {}

    let post = Post {
        title: "Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Hello World!".to_string(),
    };
    println!("{}", post.summarize());

    let weibo = Weibo {
        _username: "sunface".to_string(),
        _content: "unknown".to_string(),
    };
    println!("{}", weibo.summarize())
}

#[test]
fn it_trait_method_for_generic() {
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("the largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest::<char>(&char_list);
    println!("The largest char is {}", result);
}

#[test]
fn it_impl_add_trait() {
    use std::ops::Add;

    // #1. 两个相同的类型（Point）相加，默认泛型类型参数 trait Add<RHS=Self>
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;
        // self => point 实例
        // Self => Point<T> 类型
        fn add(self, other: Self) -> Self::Output {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let result = p1 + p2;
    println!("{:?}", result);

    // #2. 两个不同的类型相加
    #[derive(Debug)]
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, other: Meters) -> Self::Output {
            Millimeters(self.0 + (other.0 / 1000))
        }
    }

    let meters = Meters(2000);
    let millimeters = Millimeters(3);
    let result = millimeters + meters;
    println!("result: {:?}", result);
}

#[test]
fn it_trait_method_for_add() {
    use std::ops::Add;
    #[derive(Debug)]
    struct Point<T: Add<T, Output = T>> {
        x: T,
        y: T,
    }

    impl<T: Add<T, Output = T>> Add for Point<T> {
        type Output = Point<T>;
        fn add(self, p: Self) -> Self::Output {
            Point {
                x: self.x + p.x,
                y: self.y + p.y,
            }
        }
    }

    fn add<T: Add<T, Output = T>>(x: T, y: T) -> T {
        x + y
    }

    let p1 = Point { x: 1i32, y: 1i32 };
    let p2 = Point { x: 2i32, y: 2i32 };
    println!("{:?}", add(p1, p2));

    let p3 = Point::<f32> { x: 1.1, y: 1.1 };
    let p4 = Point::<f32> { x: 2.1, y: 2.1 };
    println!("{:.2?}", add(p3, p4));
}

#[test]
fn it_trait_method_for_cmp() {
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x: x, y: y }
        }
    }

    impl<T: std::fmt::Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x > self.y {
                println!("the largest member is x = {}", self.x);
            } else {
                println!("the largest member is y = {}", self.y);
            }
        }
    }

    let p = Pair::<i16>::new(10, 20);
    p.cmp_display();
}

#[test]
fn it_super_trait() {
    use std::fmt::Display;

    trait OutlinePrint: Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }
    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    // Point 必须实现 Display 特征
    impl OutlinePrint for Point {}

    let p = Point { x: 1, y: 2 };
    p.outline_print();
}

//
// trait object
//

#[test]
fn it_trait_object_01() {
    trait Draw {
        fn draw(&self);
    }

    struct Button {
        width: u32,
        height: u32,
        label: String,
    }
    impl Draw for Button {
        fn draw(&self) {
            println!(
                "Button: width={}, height={}, label={}",
                self.width, self.height, self.label
            )
        }
    }

    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }
    impl Draw for SelectBox {
        fn draw(&self) {
            println!(
                "SelectBox: width={}, height={}, options:{:?}",
                self.width, self.height, self.options
            )
        }
    }

    struct Screen {
        components: Vec<Box<dyn Draw>>,
    }
    impl Screen {
        fn run(&self) {
            for component in self.components.iter() {
                component.draw();
            }
        }
    }

    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
        ],
    };
    screen.run();
}

#[test]
fn it_ipaddrs_by_trait_object() {
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
fn it_ipaddrs_by_enum() {
    #[derive(Debug)]
    enum IPAddr {
        V4(String),
        V6(String),
    }

    fn show_ip(ip: &IPAddr) {
        println!("{:?}", ip);
    }

    let v = vec![
        IPAddr::V4(String::from("127.0.0.1")),
        IPAddr::V6("::1".to_string()),
    ];
    for ip in &v {
        show_ip(ip);
    }
}
