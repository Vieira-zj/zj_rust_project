//
// 闭包 Closure
//

#[test]
fn it_clousre_sample() {
    use std::thread;
    use std::time::Duration;

    fn workout(intensity: u32, rand_num: u32) {
        let action = || {
            println!("muuuu......");
            thread::sleep(Duration::from_secs(2));
            intensity
        };

        if intensity < 25 {
            println!("做 {} 个俯卧撑", action());
        } else if rand_num == 3 {
            println!("今天休息")
        } else {
            println!("跑步 {} 分钟", action())
        }
    }

    workout(7, 10);
}

#[test]
fn it_cacher_by_closure() {
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        query: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(query: T) -> Cacher<T> {
            Cacher {
                query: query,
                value: None,
            }
        }

        // 先查询缓存值 self.value, 若不存在，则调用 query 加载
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => {
                    println!("get cached value");
                    v
                }
                None => {
                    println!("query value and cache");
                    let v = (self.query)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    let mut c = Cacher::new(|x| x + 1);
    println!("get value {}", c.value(3));
    println!("get value {}", c.value(3));
}

#[test]
fn it_fnonce_closure() {
    // 必须实现 Copy 特征
    fn run_once<F>(func: F)
    where
        F: FnOnce(usize) -> bool + Copy,
    {
        println!("{}", func(3));
        println!("{}", func(4));
    }

    let v = vec![1, 2, 3];
    run_once(|z| z == v.len());
}

#[test]
fn it_fnmut_closure() {
    // 使用了 s 的可变借用，要声明为 mut f
    fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
        f("hello")
    }

    let mut s = String::new();
    let update_str = |str| s.push_str(str);
    exec(update_str);
    println!("update str: {}", s)
}

#[test]
fn it_fn_closure() {
    fn exec<F: Fn(String)>(f: F) {
        f("world".to_string())
    }

    let s = String::from("hello");
    let print_str = |str| println!("{}, {}", s, str);
    exec(print_str);
    println!("{:?}", s);
}

#[test]
fn it_return_fn_closure() {
    fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
        let num = 5;
        if x > 5 {
            return Box::new(move |a| a - num);
        } else {
            return Box::new(move |a| a + num);
        }
    }

    let f = factory(1);
    let result = f(2);
    println!("result: {}", result);
}

//
// 迭代器 Iterator
//

#[test]
fn it_iterator_common() {
    // into_iter 把数组转换成迭代器
    let arr = [1, 2, 3];
    for v in arr.into_iter() {
        println!("{}", v);
    }
    println!();

    // next()
    let mut iter = arr.into_iter();
    loop {
        match iter.next() {
            Some(v) => println!("{}", v),
            None => break,
        }
    }
}

#[test]
fn it_iter_methods() {
    // #1. into_iter 会夺走所有权
    let vect = vec![1, 2, 3];
    for v in vect.into_iter() {
        println!("{}", v);
    }
    // 下面的代码将报错，因为 vect 的所有权在上面 for 循环中已经被转移走
    // println!("{:?}", vect);
    println!();

    // #2. iter 是借用
    let vect = vec![1, 2, 3];
    for v in vect.iter() {
        println!("{}", v);
    }
    println!("{:?}", vect);
    println!();

    // #3. iter_mut 是可变借用
    let mut vect = vec![1, 2, 3];
    let mut iter_mut = vect.iter_mut();
    if let Some(v) = iter_mut.next() {
        *v = 0;
    }
    println!("{:?}", vect);
}

#[test]
fn it_iterator_collect() {
    // vector collect
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);

    // hashmap collect
    use std::collections::HashMap;
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    println!("{:?}", folks);

    // enumerate
    let v = vec![1u64, 2, 3, 4, 5, 6];
    let result = v
        .iter()
        .enumerate()
        .filter(|&(idx, _)| idx % 2 == 0)
        .map(|(_, val)| val)
        .fold(0u64, |sum, &val| sum + val); // 1+3+5 = 9
    println!("result: {}", result);
}

#[test]
fn it_custom_iterator() {
    struct Counter {
        value: u32,
    }

    impl Counter {
        fn new() -> Self {
            Counter { value: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.value < 5 {
                self.value += 1;
                Some(self.value)
            } else {
                None
            }
        }
    }

    let counter = Counter::new();
    for val in counter {
        println!("value: {}", val)
    }

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b) // [2, 6, 12, 20]
        .filter(|x| x % 3 == 0) // [6, 12]
        .sum();
    println!("sum: {}", sum);
}

//
// 深入类型
//

#[test]
fn it_type_alias_sample() {
    // 类型别名仅仅是别名，newtype 是全新的类型
    type Meters = u32;

    let x: u32 = 5;
    let y: Meters = 10;
    println!("x + y = {}", x + y);
}

#[test]
fn it_newtype_for_vec() {
    use std::fmt;

    // 为 Vec 实现 Display 特征
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(" ,"))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("wrapper: {}", w);
}

#[test]
fn it_newtype_for_custom_type() {
    use std::fmt;
    use std::ops::Add;

    struct Meters(u32);

    impl fmt::Display for Meters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "distance: {} meters", self.0)
        }
    }

    impl Add for Meters {
        type Output = Meters;
        fn add(self, rhs: Self) -> Self::Output {
            Self(self.0 + rhs.0)
        }
    }

    fn calculate_distance(d1: Meters, d2: Meters) -> Meters {
        d1 + d2
    }

    let d = calculate_distance(Meters(10), Meters(20));
    println!("{}", d)
}

#[test]
fn it_sized_trait() {
    use std::fmt;

    // 特征 ?Sized 用于表明类型 T 既有可能是固定大小的类型，也可能是动态大小的类型
    // 函数参数类型从 T 变成了 &T, 因为 T 可能是动态大小的，因此需要用一个固定大小的指针（引用）来包裹它
    fn my_print<T: ?Sized + fmt::Display>(s: &T) {
        println!("{}", s)
    }

    let s = "hello";
    my_print(s);
}

#[test]
fn it_tryinto_int_to_enum() {
    use std::convert::TryFrom;

    enum MyEnum {
        A = 1,
        B,
        C,
    }

    impl TryFrom<i32> for MyEnum {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            match value {
                x if x == MyEnum::A as i32 => Ok(MyEnum::A),
                x if x == MyEnum::B as i32 => Ok(MyEnum::B),
                x if x == MyEnum::C as i32 => Ok(MyEnum::C),
                _ => Err(()),
            }
        }
    }

    let x = MyEnum::C as i32;
    match x.try_into() {
        Ok(MyEnum::A) => println!("a"),
        Ok(MyEnum::B) => println!("b"),
        Ok(MyEnum::C) => println!("c"),
        Err(_) => eprintln!("unknown number"),
    }
}

#[test]
fn it_transmute_int_to_enum() {
    // 使用 #[repr(..)] 来控制底层类型的大小，免得本来需要 i32, 结果传入 i64, 最终内存无法对齐，产生非预期的结果
    #[repr(i32)]
    enum MyEnum {
        _A = 1,
        _B,
        C,
    }

    let x = MyEnum::C;
    let y = x as i32;
    let z: MyEnum = unsafe { std::mem::transmute(y) };

    // 数值一定不会超过枚举的范围
    match z {
        MyEnum::_A => println!("found A"),
        MyEnum::_B => println!("found B"),
        MyEnum::C => println!("found C"),
    }
}

//
// 智能指针
// https://course.rs/advance/smart-pointer/intro.html
//

// TODO:
