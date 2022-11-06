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
// TODO:

//
// trait
//

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
