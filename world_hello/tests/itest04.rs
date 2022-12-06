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

#[test]
fn it_pointer_box_in_vec() {
    let arr = vec![Box::new(1), Box::new(2)];
    // 使用 & 借用数组中的元素，否则会报所有权错误
    let (first, second) = (&arr[0], &arr[1]);
    // 使用 ** 做两次解引用，第一次将 &Box<i32> 类型转成 Box<i32>, 第二次将 Box<i32> 转成 i32
    let sum = **first + **second;
    println!("sum: {}", sum);
}

#[test]
fn it_pointer_box_leak() {
    fn gen_static_str() -> &'static str {
        let mut s = String::new();
        s.push_str("hello, world");
        Box::leak(s.into_boxed_str())
    }

    let s = gen_static_str();
    println!("static string: {}", s);
}

#[test]
fn it_defer_ref() {
    let x = 5;
    let y = &x;
    let z = &x;
    assert_eq!(5, *y);
    println!("y={}", *y);
    println!("z={}", z);

    let b = Box::new(1);
    let sum = *y + *b;
    println!("sum: {}", sum);
}

#[test]
fn it_deref_in_method_args() {
    fn display(s: &str) {
        println!("display: {}", s);
    }

    let b = Box::new(String::from("hello"));
    assert_eq!("hello", *b);
    // MyBox => String => &str
    // 必须使用 &s 的方式来触发 Deref
    display(&b);
}

#[test]
fn it_custom_deref_struct() {
    use std::ops::Deref;

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            Self(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            // 返回常规引用
            &(self.0)
        }
    }

    fn print_box_value(x: i32) {
        println!("box value: {}", x);
    }

    fn print_box_value_by_ref(x: &i32) {
        println!("box value: {}", x);
    }

    let x = MyBox::new(5);
    assert_eq!(5, *x);

    print_box_value(*x);
    print_box_value_by_ref(&x);
}

#[test]
fn it_custom_derefmut_struct() {
    use std::ops::Deref;
    use std::ops::DerefMut;

    struct MyBox<T> {
        v: T,
    }

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            Self { v: x }
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &(self.v)
        }
    }

    // 要实现 DerefMut 必须要先实现 Deref 特征
    impl<T> DerefMut for MyBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.v
        }
    }

    fn display(s: &mut String) {
        s.push_str(" world");
        println!("display: {}", s);
    }

    let mut b = MyBox::new(String::from("hello"));
    // &mut MyBox<String> => &mut String
    display(&mut b);
}

#[test]
fn it_custom_drop_struct() {
    struct HasDrop1;
    impl Drop for HasDrop1 {
        fn drop(&mut self) {
            println!("Dropping HasDrop1!");
        }
    }

    struct HasDrop2;
    impl Drop for HasDrop2 {
        fn drop(&mut self) {
            println!("Dropping HasDrop2!");
        }
    }

    struct HasTwoDrops {
        _one: HasDrop1,
        _two: HasDrop2,
    }
    impl Drop for HasTwoDrops {
        fn drop(&mut self) {
            println!("Dropping HasTwoDrops!");
        }
    }

    struct Foo;
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Dropping Foo!")
        }
    }

    let _x = HasTwoDrops {
        _two: HasDrop2 {},
        _one: HasDrop1 {},
    };
    let _f = Foo {};
    println!("running...");
}

#[test]
fn it_pointer_rc_clone() {
    use std::rc::Rc;

    let a = Rc::new(String::from("hello"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&b));

    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

#[test]
fn it_pointer_rc_sample() {
    use std::rc::Rc;

    struct Owner {
        name: String,
    }

    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
    }

    let gadget_owner = Rc::new(Owner {
        name: "Foo".to_string(),
    });

    let gadget1 = Gadget {
        id: 1,
        owner: Rc::clone(&gadget_owner),
    };
    let gadget2 = Gadget {
        id: 2,
        owner: Rc::clone(&gadget_owner),
    };

    // 存在 3 个指向 Foo 的智能指针引用，这里仅仅 drop 掉其中 1 个智能指针引用（不是 drop 掉 owner 数据）
    // 仍然还有 2 个引用指向底层的 owner 数据
    drop(gadget_owner);

    println!("gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    println!("gadget {} owned by {}", gadget2.id, gadget2.owner.name);

    println!("ref count: {}", Rc::strong_count(&gadget1.owner));
}
