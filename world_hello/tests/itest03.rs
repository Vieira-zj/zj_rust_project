//
// Vector
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

// TODO:
