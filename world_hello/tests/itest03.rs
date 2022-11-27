//
// Vector
//
// array 由于它的元素类型大小固定，且长度也是固定，因此数组 array 是存储在栈上。
// vector 是存储在堆上，因此长度可以动态改变。
// array 与 vector 的关系跟 &str 与 String 的关系很像，前者是长度固定的字符串切片，后者是可动态增长的字符串。
//

#[test]
fn it_vector_common_01() {
    // create
    let arr: [u8; 3] = [1, 2, 3];
    let mut v = Vec::<u8>::new();
    for i in &arr {
        v.push(*i);
    }
    println!("{:?}", v);

    let v = Vec::from(arr);
    println!("{:?}", v);
    println!();

    // get
    let v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    println!("3rd item: {}", third);

    match v.get(10) {
        Some(third) => println!("10th item: {}", third),
        None => println!("10th item not exist"),
    }
    println!();

    // iterator
    let mut v = vec![1, 2, 3];
    print!("value: ");
    for i in &v {
        print!("{},", i);
    }
    println!();

    for i in &mut v {
        *i += 10;
    }

    print!("new value: ");
    for i in v.iter() {
        print!("{},", i);
    }
    println!();
}

#[test]
fn it_vector_common_02() {
    // extend
    let mut v1 = Vec::from([1, 2, 4]);
    v1.pop();
    v1.push(3);
    println!("{:?}", v1);

    let mut v2 = Vec::new();
    v2.extend([1, 2, 3]);
    println!("{:?}", v2);
    println!();

    // array => vector
    // 只要为 Vec 实现了 From<T> 特征，那么 T 就可以被转换成 Vec
    let arr = [1, 2, 3, 4, 5];
    let v1 = Vec::from(arr);
    println!("{:?}", v1);
    let v2: Vec<i8> = arr.into();
    println!("{:?}", v2);
    println!();

    // string => vector
    let s = "abc".to_string();
    let v1: Vec<u8> = s.into();
    println!("{:?}", v1);

    let s = "abc".to_string();
    let v2 = s.into_bytes();
    println!("{:?}", v2);

    let s = "abc";
    let v3 = Vec::from(s);
    println!("{:?}", v3);
}

#[test]
fn it_vector_common_03() {
    // index
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        if let Some(x) = v.get(i) {
            v[i] = x + 1;
        } else {
            v.push(i + 2);
        }
    }
    println!("{:?}", v); // [2, 3, 4, 5, 6]
    println!();

    // capacity
    // 容量调整策略是加倍，例如 2 -> 4 -> 8
    let mut v = Vec::<i32>::with_capacity(10);
    println!("len={}, cap={}", v.len(), v.capacity());

    for i in 0..10 {
        v.push(i);
    }
    println!("len={}, cap={}", v.len(), v.capacity());
    v.push(11);
    println!("len={}, cap={}", v.len(), v.capacity());
    println!("");

    // slice
    // 当一个函数只需要可读性时，那传递 Vec 或 String 的切片 &[T] / &str 会更加适合
    let mut v = vec![1, 2, 3];
    let s1 = &v[..];
    println!("{:?}", s1);

    let v_ref = &mut v;
    // 根据方法定义 push(&mut self, value: T) 中的 self 声明，v_ref 做隐式转换
    v_ref.push(4);
    (*v_ref).push(5);
    let s2 = &v_ref[..v_ref.len()];
    println!("{:?}", s2);
}

//
// HashMap
//

#[test]
fn it_hashmap_common_01() {
    use std::collections::HashMap;
    // create
    let teams_list = vec![
        ("China".to_string(), 100),
        ("US".to_string(), 50),
        ("Japan".to_string(), 10),
    ];
    // HashMap<_, _> => 由编译器推导 kv 类型
    let mut teams_map: HashMap<_, _> = teams_list.into_iter().collect();
    println!("{:?}", teams_map);

    // get
    let team_name = "China".to_string();
    match teams_map.get(&team_name) {
        Some(score) => println!("score: {}", score),
        None => println!("not found"),
    }
    println!();

    // insert
    teams_map.insert("England".to_string(), 30);
    teams_map.insert("US".to_string(), 55); // update existing value

    // 若不存在则插入新值
    teams_map.entry("Brazil".to_string()).or_insert(70);
    teams_map.entry("Brazil".to_string()).or_insert(75);

    // iterator
    for (key, value) in &teams_map {
        println!("{}: {}", key, value);
    }
}

#[test]
fn it_hashmap_common_02() {
    use std::collections::HashMap;
    let mut scores = HashMap::<&str, i32>::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69);
    scores.insert("Katie", 58);

    let score = scores.get("Sunface");
    assert_eq!(score, Some(&98));

    if scores.contains_key("Daniel") {
        let score = scores["Daniel"];
        assert_eq!(score, 95);
        scores.remove("Daniel");
    }
    assert_eq!(scores.len(), 3);

    for (name, score) in scores {
        println!("{}: {}", name, score);
    }
}

/*
HashMap key
- int, uint 以及它们的变体，例如 u8, i32 等
- String 和 &str

注意：f32 和 f64 不能作为 HashMap key, 原因是它们并没有实现 Hash, 浮点数精度 的问题会导致它们无法进行相等比较。
*/

#[test]
fn it_custom_hashmap_key() {
    use std::collections::HashMap;
    #[derive(Debug, Hash, PartialEq, Eq)]
    struct CustomKey {
        name: String,
        country: String,
    }

    impl CustomKey {
        fn new(name: &str, country: &str) -> Self {
            CustomKey {
                name: name.to_string(),
                country: country.to_string(),
            }
        }
    }

    let map = HashMap::from([
        (CustomKey::new("Einar", "Norway"), 25),
        (CustomKey::new("Olaf", "Denmark"), 24),
        (CustomKey::new("Harald", "Iceland"), 12),
    ]);
    for (key, value) in map {
        println!("{:?}: {}", key, value);
    }
}

#[test]
fn it_hashmap_capacity() {
    use std::collections::HashMap;
    // new() => 会设置一个默认的初始化容量
    let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
    map.insert(1, 2);
    map.insert(3, 4);
    // 虽然我们使用了 100 容量来初始化，但是 map 的容量很可能会比 100 更多
    assert!(map.capacity() >= 100);

    // 缩容。这个值会尽量靠近你提供的值，同时还可能会预留一些调整空间
    map.shrink_to(50);
    assert!(map.capacity() >= 50);

    // 自行调整到一个合适的值
    map.shrink_to_fit();
    assert!(map.capacity() >= 2);
    println!("size: {}", map.capacity());
}

//
// 错误处理
//

#[test]
#[should_panic(expected = "No such file or directory")]
fn it_unwrap_error_handle() {
    use std::fs::File;
    let path = "/tmp/test/log.txt";
    let _ = File::open(path).unwrap();
}

#[test]
#[should_panic(expected = "failed to open")]
fn it_expect_error_handle() {
    use std::fs::File;
    let path = "/tmp/test/log.txt";
    // expect 自定义错误提示信息
    let _ = File::open(path).expect(&format!("failed to open {}", path));
}

#[test]
fn it_match_error_kind() {
    use std::fs::File;
    use std::io::ErrorKind;

    let path = "/tmp/test/log.txt";
    let f = File::open(path);
    let _ = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => {
                    println!("create file: {}", path);
                    fc
                }
                Err(err) => panic!("problem creating file: {:?}", err),
            },
            other_error => panic!("problem opening file: {:?}", other_error),
        },
    };
}

#[test]
fn it_fread_result_handle() {
    use std::fs::File;
    use std::io;
    use std::io::Read;

    fn read_from_file() -> Result<String, io::Error> {
        let f = File::open("/tmp/test/log.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e), // 直接返回错误
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    match read_from_file() {
        Ok(s) => println!("read content:\n{}", s),
        Err(e) => println!("read error: {}", e),
    }
}

#[test]
fn it_fread_throw_error() {
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

//
// 生命周期
//
// 在通过函数签名指定生命周期参数时，我们并没有改变传入引用或者返回引用的真实生命周期，而是告诉编译器当不满足此约束条件时，就拒绝编译通过。
//
// 生命周期 'static 意味着能和程序活得一样久，例如字符串字面量和特征对象。
//

#[test]
fn it_lifetime_in_func() {
    // 返回值的生命周期与参数生命周期中的较小值一致
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let str1 = String::from("abcd");
    let str2 = "xyz";
    let result = longest(str1.as_str(), str2);
    println!("longest string is {}", result);
}

#[test]
fn it_lifetime_in_struct() {
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        _part: &'a str,
    }

    impl<'a> ImportantExcerpt<'a> {
        fn _level(&self) -> i32 {
            3
        }
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        _part: first_sentence,
    };
    println!("{:?}", i)
}

#[test]
fn it_lifetime_and_generic() {
    use std::fmt::Display;
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("announcement: {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let str1 = String::from("abcd");
    let str2 = "xyz";
    let ann = "it's a lifetime and generic test";
    let result = longest_with_an_announcement(str1.as_str(), str2, ann);
    println!("longest string is {}", result);
}

// https://course.rs/advance/lifetime/advance.html

#[test]
fn it_lifetime_adv_hrbt() {
    // 生命周期约束
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    // &self 生命周期是 'a, 那么 self.part 的生命周期也是 'a
    // 由于 &'a self 是被引用的一方，因此引用它的 &'b str 必须要活得比它短，否则会出现悬垂引用
    impl<'a: 'b, 'b> ImportantExcerpt<'a> {
        fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
            println!("attention please: {}", announcement);
            self.part
        }
    }

    let i = ImportantExcerpt {
        part: "lifetime sample",
    };
    let part = i.announce_and_return_part("it test");
    println!("{}", part);
}

#[test]
fn it_lifetime_adv_reborrow() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn move_to(&mut self, x: i32, y: i32) {
            self.x = x;
            self.y = y;
        }
    }

    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;

    // reborrow
    let rr = &*r;
    println!("{:?}", rr);

    r.move_to(10, 10);
    println!("{:?}", r)
}

#[test]
fn it_lifetime_adv_sample() {
    struct Manager<'a> {
        text: &'a str,
    }

    struct Interface<'b, 'a: 'b> {
        _manager: &'b mut Manager<'a>,
    }
    impl<'b, 'a: 'b> Interface<'b, 'a> {
        pub fn noop(self) {
            println!("interface consumed");
        }
    }

    struct List<'a> {
        manager: Manager<'a>,
    }
    impl<'a> List<'a> {
        pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a>
        where
            'a: 'b,
        {
            Interface {
                _manager: &mut self.manager,
            }
        }
    }

    fn use_list(list: &List) {
        println!("{}", list.manager.text);
    }

    let mut list = List {
        manager: Manager { text: "hello" },
    };
    list.get_interface().noop();

    use_list(&list);
}

#[test]
fn it_lifetime_static() {
    use std::fmt::Display;

    let r1;
    let r2;
    {
        static STATIC_EXAMPLE: i32 = 42;
        r1 = STATIC_EXAMPLE;
        let x = "string";
        r2 = x;
    }
    // r1 和 r2 持有的数据都是 'static 的，因此在花括号结束后，并不会被释放
    println!("static i32: {}", r1);
    println!("static str: {}", r2);

    // 没有检查 T, 这里只确保 &T 的生命周期符合规则即可
    fn static_borrow<T: Display + 'static>(t: &T) {
        println!("{}", t);
    }

    let _r3: &str;
    {
        let s1 = "String".to_string();
        static_borrow(&s1);

        // s1 是 String 类型，没有 'static 的生命周期，因此下面代码会报错
        // r3 = &s1;
    }
    // println!("{}", r3);
}
