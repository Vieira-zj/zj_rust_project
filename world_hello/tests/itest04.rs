//
// 闭包 Closure
// https://course.rs/advance/functional-programing/closure.html
//

#[test]
fn it_clousre_func() {
    use std::thread;
    use std::time::Duration;

    let x = 4;
    // 闭包可以捕获作用域中的值 x
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    fn workout(intensity: u32, rand_num: u32) {
        // 这里只是把闭包赋值给变量 action, 并不是把闭包执行后的结果赋值给 action
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
fn it_closure_in_struct() {
    //  Fn 特征不仅仅适用于闭包，还适用于函数，因此上面的 query 字段除了使用闭包作为值外，还能使用一个具名函数来作为它的值
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
fn it_closure_fn_move() {
    // 当闭包的生命周期大于捕获变量的生命周期时，例如将闭包返回或移入其他线程，使用 move 关键字强制闭包取得捕获变量的所有权
    use std::thread;
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("here's a vector: {:?}", v);
    });
    handle.join().unwrap();
    println!("done")
}

#[test]
fn it_closure_type_fnonce_01() {
    // FnOnce 表明该闭包只能运行一次，该类型的闭包会拿走被捕获变量的所有权
    fn run_once<F>(func: F)
    where
        F: FnOnce(usize) -> bool + Copy,
    {
        // 仅实现 FnOnce 特征的闭包在调用时会转移所有权
        println!("{}", func(3));
        // 二次调用必须实现 Copy 特征
        println!("{}", func(4));
    }

    let v = vec![1, 2, 3];
    run_once(|z| z == v.len());
}

#[test]
fn it_closure_type_fnonce_02() {
    // 闭包从捕获环境中移出了变量 s 的所有权，因此这个闭包仅自动实现了 FnOnce, 未实现 FnMut 和 Fn
    fn exec<'a, F: FnOnce(&'a str) -> String>(f: F) {
        let res = f(", world");
        println!("{}", res);
    }

    let mut s = String::from("hello");
    let update_string = |str| {
        s.push_str(str);
        s
    };
    exec(update_string);
}

#[test]
fn it_closure_type_fnmut_01() {
    let mut s = String::new();
    // 想要在闭包内部捕获可变借用，需要把该闭包声明为可变类型
    let mut update_string = |str| s.push_str(str);
    update_string("hello");
    println!("{}", s);
}

#[test]
fn it_closure_type_fnmut_02() {
    // FnMut 它以可变借用的方式捕获了环境中的值
    // 这里使用了 s 的可变借用，要声明为 mut f
    fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
        f("hello")
    }

    let mut s = String::new();
    let update_str = |str| s.push_str(str);
    exec(update_str);
    println!("update str: {}", s);
}

#[test]
fn it_closure_type_fn_01() {
    // Fn 它以不可变借用的方式捕获环境中的值
    fn exec<F: Fn(String)>(f: F) {
        f("world".to_string())
    }

    let s = "hello".to_string();
    let print_str = |str| println!("{}, {}", s, str);
    exec(print_str);
    println!("{:?}", s);
}

#[test]
fn it_closure_type_fn_02() {
    // Fn 获取 &self, FnMut 获取 &mut self, 而 FnOnce 获取 self
    fn exec<'a, F: Fn(&'a str)>(f: F) {
        f("world")
    }

    let s = "hello";
    let print_str = |str| println!("{}, {}", s, str);
    exec(print_str);
}

#[test]
fn it_closure_return_fn() {
    // 使用 Box 返回特征对象
    // FnOnce 要求最为严格（获取 self），肯定能执行，因此这里先使用 Fn
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
fn it_iterator_basic() {
    // next()
    let values = vec![1, 2, 3];
    {
        let result = match IntoIterator::into_iter(values) {
            mut iter => loop {
                match iter.next() {
                    Some(x) => {
                        println!("{}", x);
                    }
                    None => break,
                }
            },
        };
        result
    }
    println!();

    // into_iter
    let arr = [1, 2, 3];
    let arr_iter = arr.into_iter();
    for v in arr_iter {
        println!("{}", v);
    }
    println!("{:?}", arr);
    println!();

    // 惰性初始化
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }
}

#[test]
fn it_iter_into_iterator() {
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
    // 取出第一个元素，并修改为 0
    if let Some(v) = iter_mut.next() {
        *v = 0;
    }
    println!("{:?}", vect);
}

#[test]
fn it_iterator_collect() {
    // #1. sum
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // sum 是消费性适配器，它会拿走迭代器的所有权，然后通过不断调用 next 方法对里面的元素进行求和
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    // v1_iter 是借用了 v1, 因此 v1 可以照常使用
    println!("{:?}", v1);
    // 以下代码会报错，因为 sum 拿走了迭代器 v1_iter 的所有权
    // println!("{:?}", v1_iter);
    println!();

    // #2. vector collect
    // map 是迭代器适配器，惰性的，意味着你需要一个消费者适配器来收尾
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);
    println!();

    // #3. hashmap collect
    use std::collections::HashMap;
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: HashMap<_, _> = names.into_iter().zip(ages.into_iter()).collect();
    println!("{:?}", folks);
}

#[test]
fn it_iterator_enumerate() {
    // enumerate 产生一个新的迭代器，其中每个元素均是元组 (index,value)
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
fn it_closure_in_iterator() {
    #[derive(Debug)]
    struct Shoe {
        size: u32,
        _style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {
            size: 42,
            _style: "football".to_string(),
        },
        Shoe {
            size: 45,
            _style: "basketball".to_string(),
        },
    ];
    let results = shoes_in_size(shoes, 45);
    println!("{:?}", results);
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
        type Item = u32; // 关联类型
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
        println!("value: {}", val);
    }
    println!();

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1)) // [2, 3, 4, 5]
        .map(|(a, b)| a * b) // [2, 6, 12, 20]
        .filter(|x| x % 3 == 0) // [6, 12]
        .sum();
    println!("sum: {}", sum);
}

//
// 深入类型
//

#[test]
fn it_type_convert_unsafe_ptr() {
    // 内存地址转换为指针
    let mut values = [1, 2];
    let p1 = values.as_mut_ptr();
    let first_addr = p1 as usize;
    let second_addr = first_addr + 4;
    let p2 = second_addr as *mut i32;
    unsafe {
        *p2 += 1;
    }
    println!("values: {:?}", values);
}

#[test]
#[should_panic(expected = "TryFromIntError")]
fn it_type_convert_tryinto() {
    let b: i16 = 1500;
    let b_: i8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("convert error: {}", e.to_string());
            0
        }
    };
    println!("b_: {}", b_);

    // 使用 unwrap 方法，该方法在发现错误时，会直接调用 panic 导致程序的崩溃退出
    let b_: i8 = b.try_into().unwrap();
    println!("b_: {}", b_);
}

#[test]
fn it_newtype_type_alias() {
    // 类型别名仅仅是别名，而 newtype 是全新的类型
    type Meters = u32;

    let x: u32 = 5;
    let y: Meters = 10;
    println!("x + y = {}", x + y);
}

#[test]
fn it_define_newtype_wrapper() {
    use std::fmt;

    // newtype 使用元组结构体的方式将已有的类型包裹起来
    struct Wrapper(Vec<String>);
    // 为 Vec 实现 Display 特征
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "[{}]", self.0.join(" ,"))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("wrapper: {}", w);
}

#[test]
fn it_define_newtype_meters() {
    use std::fmt;
    use std::ops::Add;

    struct Meters(u32);
    impl fmt::Display for Meters {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "目标地点距离你 {} 米", self.0)
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
    println!("{}", d);
}

#[test]
fn it_type_sized_trait() {
    use std::fmt;

    // Rust 中常见的 DST 类型有: str, [T], dyn Trait, 它们都无法单独被使用，必须要通过引用或者 Box 来间接使用
    // 而在编译时就能知道其大小的类型，都会自动实现 Sized 特征

    // 特征 ?Sized 用于表明类型 T 既有可能是固定大小的类型，也可能是动态大小的类型
    // 函数参数类型从 T 变成了 &T, 因为 T 可能是动态大小的，因此需要用一个固定大小的指针（引用）来包裹它
    fn my_print<T: ?Sized + fmt::Display>(s: &T) {
        println!("{}", s)
    }

    let s = "hello";
    my_print(s);
}

#[test]
fn it_type_convert_tryfrom() {
    use std::convert::TryFrom;
    use std::convert::TryInto;

    enum MyEnum {
        A = 1,
        B,
        C,
    }

    // 实现 TryFrom 特征
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

    // 整数转换为枚举
    // 使用 try_into 来实现转换
    let x = MyEnum::C as i32;
    match x.try_into() {
        Ok(MyEnum::A) => println!("a"),
        Ok(MyEnum::B) => println!("b"),
        Ok(MyEnum::C) => println!("c"),
        Err(_) => eprintln!("unknown number"),
    }
}

#[test]
fn it_type_convert_transmute() {
    // 使用 #[repr(..)] 来控制底层类型的大小，本来需要 i32, 结果传入 i64, 最终内存无法对齐，产生非预期的结果
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
fn it_pointer_box() {
    {
        // 在栈上创建一个长度为 100 的数组
        let arr = [0; 100];
        // 新深拷贝一份数据
        let arr1 = arr;
        println!("{:?}", arr.len());
        println!("{:?}", arr1.len());
        println!();
    }

    {
        // 在堆上创建一个长度为 100 的数组，然后使用一个智能指针指向它
        let arr = Box::new([0; 100]);
        // 将堆上数组的所有权转移给 arr1, 由于数据在堆上，因此仅仅拷贝了智能指针的结构体，底层数据并没有被拷贝
        // 所有权顺利转移给 arr1, arr 不再拥有所有权
        let arr1 = arr;
        println!("{:?}", arr1.len());
        // 由于 arr 不再拥有底层数组的所有权，因此下面代码将报错
        // println!("{:?}", arr.len());
        println!();
    }

    let arr = vec![Box::new(1), Box::new(2)];
    // 使用 & 借用数组中的元素，否则会报所有权错误
    let (first, second) = (&arr[0], &arr[1]);
    // 表达式不能隐式的解引用，因此必须使用 ** 做两次解引用，第一次将 &Box<i32> 类型转成 Box<i32>, 第二次将 Box<i32> 转成 i32
    let sum = **first + **second;
    println!("sum: {}", sum);
}

#[test]
fn it_pointer_box_leak() {
    // 使用 Box::leak 可以将一个运行期的值转为 'static
    fn gen_static_str() -> &'static str {
        let mut s = String::new();
        s.push_str("hello, world");
        Box::leak(s.into_boxed_str())
    }

    let s = gen_static_str();
    println!("static string: {}", s);
}

#[test]
fn it_pointer_deref() {
    let x = 5;
    let y = &x;
    let z = &x;
    assert_eq!(5, *y);
    println!("y={}", *y);
    println!("z={}", z);
    println!();

    // 通过 * 进行解引用
    let b = Box::new(1);
    let sum = *y + *b;
    println!("sum: {}", sum);
}

#[test]
fn it_impl_deref_fn_args() {
    fn display(s: &str) {
        println!("display: {}", s);
    }

    {
        let s = String::from("hi");
        // &s 是一个 &String 类型，当它被传给 display 函数时，自动通过 Deref 转换成了 &str
        // String 实现了 Deref 特征，必须使用 &s 的方式来触发 Deref
        display(&s);
    }

    {
        let b = Box::new(String::from("hello"));
        assert_eq!("hello", *b);
        // 连续的隐式 Deref 转换：MyBox => String => &str
        display(&b);
    }
}

#[test]
fn it_pointer_custom_deref() {
    use std::ops::Deref;

    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            Self(x)
        }
    }
    impl<T> Deref for MyBox<T> {
        type Target = T;
        // 返回的是一个常规引用，可以被 * 进行解引用
        fn deref(&self) -> &Self::Target {
            &self.0
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
fn it_pointer_custom_derefmut() {
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
            &self.v
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
    // 这里将 &mut MyBox<String> 转换为 &mut String
    display(&mut b);
}

#[test]
fn it_pointer_impl_drop() {
    struct HasDrop1;
    impl Drop for HasDrop1 {
        // 这里借用了目标的可变引用
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

// RC (reference counting)
// - Rc/Arc 是不可变引用，你无法修改它指向的值
// - 一旦最后一个拥有者消失，则资源会自动被回收
// - Rc 只能用于同一线程内部

#[test]
fn it_pointer_rc_clone() {
    // clone 仅仅复制了智能指针并增加了引用计数，并没有克隆底层数据
    use std::rc::Rc;

    let a = Rc::new(String::from("hello"));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&b));

    {
        let c = b.clone();
        println!("count after creating c = {}", Rc::strong_count(&c));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    println!("{}, {}", a, &b);
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

    // 存在 3 个指向 owner 的智能指针引用，这里仅仅 drop 掉其中 1 个智能指针引用（不是 drop 掉 owner 数据）
    // 仍然还有 2 个引用指向底层的 owner 数据
    drop(gadget_owner);

    println!("gadget {} owned by {}", gadget1.id, gadget1.owner.name);
    let name = &gadget1.owner.name;
    println!("gadget {} owned by {}", gadget2.id, name);

    println!("ref count: {}", Rc::strong_count(&gadget1.owner));
}

#[test]
fn it_pointer_atomic_rc() {
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    let s = Arc::new(String::from("shared_string_in_threads"));
    let mut handlers = Vec::with_capacity(6);
    for _ in 1..6 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(300));
            println!("{}", s)
        });
        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }
    println!("all threads done");
}

// Cell 可以在拥有不可变引用的同时修改目标数据，对于正常的代码实现来说，这个是不可能做到的（要么一个可变借用，要么多个不可变借用）。

#[test]
fn it_pointer_cell() {
    use std::cell::Cell;

    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
    y.set(3);
    z.set(4);
    println!("{}, {}, {}", x.get(), y.get(), z.get());

    // let mut x = 1;
    // let y = &mut x;
    // let z = &mut x;
    // x = 2;
    // *y = 3;
    // *z = 4;
    // println!("{}", x);
}

#[test]
#[should_panic(expected = "already borrowed: BorrowMutError")]
fn it_pointer_refcell() {
    // 与 Cell 用于可 Copy 的值不同，RefCell 用于引用
    // RefCell 只是将借用规则从编译期推迟到程序运行期，并不能帮你绕过这个规则
    use std::cell::RefCell;

    let s = String::from("hello");
    let rc = RefCell::new(s);
    let s1 = rc.borrow();
    let s2 = rc.borrow_mut();
    println!("{}, {}", s1, s2);
}

#[test]
fn it_pointer_rc_refcell() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let s = Rc::new(RefCell::new("hello".to_string()));
    let s1 = s.clone();
    let s2 = s.clone();
    s2.borrow_mut().push_str(", rust");

    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}

#[test]
fn it_pointer_refcell_sample() {
    use std::cell::RefCell;

    trait Messager {
        fn send(&self, msg: String);
    }

    // 通过包裹一层 RefCell, 成功的让 &self 中的 msg_cache 成为一个可变值，然后实现对其的修改
    struct MsgQueue {
        msg_cache: RefCell<Vec<String>>,
    }
    impl Messager for MsgQueue {
        fn send(&self, msg: String) {
            self.msg_cache.borrow_mut().push(msg)
        }
    }

    let mq = MsgQueue {
        msg_cache: RefCell::new(Vec::new()),
    };
    mq.send("hello".to_string());
    mq.send("world".to_string());
    println!("{}", mq.msg_cache.borrow().join(", ").as_str());
}

#[test]
fn it_pointer_cell_from_mut() {
    use std::cell::Cell;

    fn is_even(i: i32) -> bool {
        i % 2 == 0
    }

    fn retain_even(nums: &mut Vec<i32>) {
        // Cell::from_mut 该方法将 &mut T 转为 &Cell<T>
        // Cell::as_slice_of_cells 该方法将 &Cell<[T]> 转为 &[Cell<T>]
        let s = Cell::from_mut(&mut nums[..]).as_slice_of_cells();
        let mut i = 0;
        for num in s.iter().filter(|num| is_even(num.get())) {
            // Cell 上的 set 方法获取的是不可变引用 pub fn set(&self, val: T)
            s[i].set(num.get());
            i += 1;
        }
        nums.truncate(i);
    }

    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    retain_even(&mut nums);
    println!("even nums: {:?}", nums);
}
