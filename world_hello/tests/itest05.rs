//
// 循环引用与自引用
// https://course.rs/advance/circle-self-ref/circle-reference.html
//

#[test]
fn it_weak_ref() {
    // weak ref 可访问，但没有所有权，不增加引用计数，因此不会影响被引用值的释放回收
    use std::rc::Rc;

    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);

    let strong_five = weak_five.upgrade();
    if let Some(v) = strong_five {
        println!("{}, {}", v, *v);
    }

    // 拿走了目标值的所有权
    drop(five);

    let strong_five = weak_five.upgrade();
    assert_eq!(strong_five, None);
}

#[test]
fn it_weak_ref_sample() {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    // Owner 与 Gadget 相互引用
    struct Owner {
        name: String,
        // 需要修改 gadgets, 这里使用 RefCell
        gadgets: RefCell<Vec<Weak<Gadget>>>,
    }

    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
    }

    let gadget_owner = Rc::new(Owner {
        name: "Gadget Man".to_string(),
        gadgets: RefCell::new(Vec::new()),
    });

    let gadget1 = Rc::new(Gadget {
        id: 1,
        owner: gadget_owner.clone(),
    });
    let gadget2 = Rc::new(Gadget {
        id: 2,
        owner: gadget_owner.clone(),
    });

    // 因为之前使用了 Rc, 现在必须要使用 Weak, 否则就会循环引用
    gadget_owner
        .gadgets
        .borrow_mut()
        .push(Rc::downgrade(&gadget1));
    gadget_owner
        .gadgets
        .borrow_mut()
        .push(Rc::downgrade(&gadget2));

    for gadget_opt in gadget_owner.gadgets.borrow().iter() {
        let gadget = gadget_opt.upgrade().unwrap();
        println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
    }
}

#[test]
fn it_weak_ref_tree_sample() {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    #[derive(Debug)]
    struct Node {
        _value: i32,
        parent: RefCell<Weak<Node>>,
        _children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        _value: 3,
        parent: RefCell::new(Weak::new()),
        _children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 1 ref: leaf
        Rc::weak_count(&leaf),   // 0 weak ref
    );

    {
        let branch = Rc::new(Node {
            _value: 5,
            parent: RefCell::new(Weak::new()),
            _children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch), // 1 ref: branch
            Rc::weak_count(&branch),   // 1 weak ref: leaf
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf), // 2 ref: branch,leaf
            Rc::weak_count(&leaf),   // 0 weak ref
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 1 ref: leaf
        Rc::weak_count(&leaf),   // 0 weak ref
    );
}

#[test]
fn it_selfref_by_option() {
    #[derive(Debug)]
    struct WhatAboutThis<'a> {
        name: String,
        nickname: Option<&'a str>,
    }

    let mut tricky = WhatAboutThis {
        name: "Annabelle".to_string(),
        nickname: None,
    };
    tricky.nickname = Some(&tricky.name[..4]);

    println!("{:?}", tricky);
}

#[test]
fn it_selfref_by_unsafe_ptr() {
    #[derive(Debug)]
    struct SelfRef {
        value: String,
        // 将 *const 修改为 *mut, 通过裸指针来修改 String
        // ptr_to_value: *const String,
        ptr_to_value: *mut String,
    }

    impl SelfRef {
        fn new(txt: &str) -> Self {
            Self {
                value: String::from(txt),
                ptr_to_value: std::ptr::null_mut(),
            }
        }

        fn init(&mut self) {
            let self_ref: *mut String = &mut self.value;
            self.ptr_to_value = self_ref;
        }

        fn value(&self) -> &str {
            &self.value
        }

        fn ptr_to_value(&self) -> &String {
            assert!(
                !self.ptr_to_value.is_null(),
                "Test::b called without Test::init being called first"
            );
            unsafe { &*(self.ptr_to_value) }
        }
    }

    let mut sr = SelfRef::new("hello");
    sr.init();
    println!("{}, {:p}", sr.value(), sr.ptr_to_value());

    sr.value.push_str(", world");
    unsafe {
        (&mut *sr.ptr_to_value).push_str("!");
    }
    println!("{}, {:p}", sr.value(), sr.ptr_to_value());
}

//
// 全局变量
//

#[test]
fn it_global_id_generator_by_atomic() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    // 赋值必须是在编译期就能计算出的值，如果需要在运行时才能得出结果的值，比如函数，则不能赋值给常量表达式
    static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0); // 静态变量
    const MAX_ID: usize = usize::MAX / 2; // 常量

    fn generate_id() -> usize {
        let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
        if current_val > MAX_ID {
            panic!("Factory ids overflowed");
        }

        let next_id = GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        if next_id > MAX_ID {
            panic!("Factory ids overflowed");
        }
        next_id
    }

    struct Factory {
        factory_id: usize,
    }

    impl Factory {
        fn new() -> Self {
            Self {
                factory_id: generate_id(),
            }
        }
    }

    let mut f = Factory::new();
    println!("factory id: {}", f.factory_id);

    f = Factory::new();
    println!("factory id: {}", f.factory_id);
}

#[test]
fn it_global_cacher_by_lazy_static() {
    // lazy_static 允许我们在运行期初始化静态变量
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    lazy_static! {
        static ref HASHMAP: HashMap<u32, &'static str> = {
            let mut m = HashMap::new();
            m.insert(0, "foo");
            m.insert(1, "bar");
            m.insert(2, "baz");
            m
        };
    }

    // 首次访问 HASHMAP 的同时对其进行初始化
    println!("the value for 0 is: {}", HASHMAP.get(&0).unwrap());
    println!("the value for 1 is: {}", HASHMAP.get(&1).unwrap());
}

//
// Option,Result 处理
// https://course.rs/advance/errors.html
//

#[test]
fn it_option_or_and() {
    // Option
    let s1 = Some("some1");
    let s2 = Some("some2");
    let n: Option<&str> = None;

    assert_eq!(s1.or(s2), s1);
    assert_eq!(s1.or(n), s1);
    assert_eq!(n.or(n), n);

    assert_eq!(s1.and(s2), s2);
    assert_eq!(s1.and(n), n);

    // Result
    let ok1: Result<&str, &str> = Ok("ok1");
    let ok2: Result<&str, &str> = Ok("ok2");
    let err1: Result<&str, &str> = Err("error1");
    let err2: Result<&str, &str> = Err("error2");

    assert_eq!(ok1.or(ok2), ok1);
    assert_eq!(err1.or(ok1), ok1);
    assert_eq!(err1.or(err2), err2);

    assert_eq!(ok1.and(ok2), ok2);
    assert_eq!(err1.and(ok1), err1);
}

#[test]
fn it_option_or_else_and_then() {
    // Option
    let s1 = Some("some1");
    let s2 = Some("some2");
    let fn_some = || Some("some2");

    let n: Option<&str> = None;
    let fn_none = || None;

    assert_eq!(s1.or_else(fn_some), s1);
    assert_eq!(n.or_else(fn_some), s2);
    assert_eq!(n.or_else(fn_none), n);

    assert_eq!(s1.and_then(|_| Some("some2")), s2);
    assert_eq!(n.and_then(|_| Some("some2")), n);

    // Result
    let ok1: Result<&str, &str> = Ok("ok1");
    let ok2: Result<&str, &str> = Ok("ok2");
    let fn_ok = |_| Ok("ok2");

    let e1: Result<&str, &str> = Err("error1");
    let e2: Result<&str, &str> = Err("error2");
    let fn_err = |_| Err("error2");

    assert_eq!(ok1.or_else(fn_ok), ok1);
    assert_eq!(e1.or_else(fn_ok), ok2);
    assert_eq!(e1.or_else(fn_err), e2);

    assert_eq!(ok1.and_then(fn_ok), ok2);
    assert_eq!(e1.and_then(fn_ok), e1);
}

#[test]
fn it_option_filter() {
    let s1 = Some(3);
    let s2 = Some(6);
    let n = None;

    let fn_is_even = |x: &i8| x % 2 == 0;
    assert_eq!(s1.filter(fn_is_even), n);
    assert_eq!(s2.filter(fn_is_even), s2);
    assert_eq!(n.filter(fn_is_even), n);
}

#[test]
fn it_option_map() {
    let fn_character_count = |s: &str| s.chars().count();

    // Option
    let s1 = Some("abcde");
    let s2 = Some(5);

    let n1: Option<&str> = None;
    let n2: Option<usize> = None;

    assert_eq!(s1.map(fn_character_count), s2);
    assert_eq!(n1.map(fn_character_count), n2);

    // Result
    let ok1: Result<&str, &str> = Ok("abcde");
    let ok2: Result<usize, &str> = Ok(5);

    let e1: Result<&str, &str> = Err("abcde");
    let e2: Result<usize, &str> = Err("abcde");

    assert_eq!(ok1.map(fn_character_count), ok2);
    assert_eq!(e1.map(fn_character_count), e2);
}

#[test]
fn it_option_map_err() {
    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<&str, isize> = Ok("abcde");

    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);

    let fn_character_count = |s: &str| -> isize { s.parse().unwrap() };

    assert_eq!(o1.map_err(fn_character_count), o2);
    assert_eq!(e1.map_err(fn_character_count), e2);
}

#[test]
fn it_option_map_or() {
    const V_DEFAULT: u32 = 1;
    let ok: Result<u32, ()> = Ok(10);
    let n: Option<u32> = None;
    let fn_closure = |v: u32| v + 2;

    assert_eq!(ok.map_or(V_DEFAULT, fn_closure), 12);
    assert_eq!(n.map_or(V_DEFAULT, fn_closure), 1);
}

#[test]
fn it_option_map_or_else() {
    // Option
    let s = Some(10);
    let n: Option<i8> = None;

    let fn_default = || 1;
    let fn_closure = |v: i8| v + 2;

    assert_eq!(s.map_or_else(fn_default, fn_closure), 12);
    assert_eq!(n.map_or_else(fn_default, fn_closure), 1);

    // Result
    let ok = Ok(10);
    let e = Err(5);

    // 闭包可以对 Err 中的值进行处理，并返回一个新值
    let fn_default_for_result = |v: i8| v + 1;

    assert_eq!(ok.map_or_else(fn_default_for_result, fn_closure), 12);
    assert_eq!(e.map_or_else(fn_default_for_result, fn_closure), 6);
}

#[test]
fn it_option_ok_or() {
    const ERR_DEFAULT: &str = "error message";

    let s = Some("abcde");
    let n: Option<&str> = None;

    let ok: Result<&str, &str> = Ok("abcde");
    let e: Result<&str, &str> = Err(ERR_DEFAULT);

    assert_eq!(s.ok_or(ERR_DEFAULT), ok);
    assert_eq!(n.ok_or(ERR_DEFAULT), e);
}

#[test]
fn it_option_ok_or_else() {
    let s = Some("abcde");
    let n: Option<&str> = None;
    let fn_err_message = || "error message";

    let ok: Result<&str, &str> = Ok("abcde");
    let e: Result<&str, &str> = Err("error message");

    assert_eq!(s.ok_or_else(fn_err_message), ok);
    assert_eq!(n.ok_or_else(fn_err_message), e);
}

//
// 自定义错误
//

#[test]
fn it_custom_simple_error() {
    use std::fmt;

    // 自定义错误类型只需要实现 Debug 和 Display 特征即可，但并不是作为 Err 使用的必要条件
    #[derive(Debug)]
    struct AppError;

    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "An Error Occurred, Please Try Again!")
        }
    }

    fn produce_error() -> Result<(), AppError> {
        Err(AppError)
    }

    match produce_error() {
        Err(e) => eprintln!("{}", e), // print display msg
        _ => println!("No error"),
    }
    eprintln!("{:?}", produce_error()); // print debug msg
}

#[test]
fn it_custom_code_msg_error() {
    use std::fmt;

    struct AppError {
        code: usize,
        message: String,
    }

    impl fmt::Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let err_msg = match self.code {
                404 => "Sorry, Can not find the Page!",
                _ => "Sorry, something is wrong! Please Try Again!",
            };
            write!(f, "{}", err_msg)
        }
    }

    // 实现 Debug 特征
    impl fmt::Debug for AppError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "AppError {{ code: {}, message: {} }}",
                self.code, self.message
            )
        }
    }

    fn produce_error() -> Result<(), AppError> {
        Err(AppError {
            code: 404,
            message: "Page not found".to_string(),
        })
    }

    match produce_error() {
        Err(e) => eprintln!("{}", e),
        _ => println!("No error"),
    }

    eprintln!("{:?}", produce_error());
    eprintln!("{:#?}", produce_error());
}

#[test]
fn it_custom_from_trait_error() {
    use std::fs::File;
    use std::io::{self, Read};
    use std::num;

    // 只实现了 Debug 特征
    #[derive(Debug)]
    struct AppError {
        _kind: String,
        _message: String,
    }

    // io::Error => AppError
    impl From<io::Error> for AppError {
        fn from(err: io::Error) -> Self {
            AppError {
                _kind: "io".to_string(),
                _message: err.to_string(),
            }
        }
    }

    // num::ParseIntError => AppError
    impl From<num::ParseIntError> for AppError {
        fn from(err: num::ParseIntError) -> Self {
            AppError {
                _kind: String::from("parse int"),
                _message: err.to_string(),
            }
        }
    }

    fn open_file() -> Result<(), AppError> {
        // 这里 ? 可以将错误进行隐式的强制转换：File::open 返回的是 std::io::Error, 我们并没有进行任何显式的转换，它就能自动变成 AppError
        let mut file = File::open("/tmp/test/test.txt")?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let _number: usize;
        _number = content.parse()?;
        Ok(())
    }

    match open_file() {
        Ok(_) => println!("Success open file"),
        Err(e) => eprintln!("Failed open file: {:?}", e),
    }
}

#[test]
#[should_panic(expected = "value: NotPresent")]
fn it_generic_error_box_dyn() {
    use std::error::Error;
    use std::fs::read_to_string;

    fn render() -> Result<String, Box<dyn Error>> {
        let file = std::env::var("MARKDOWN")?; // return std::env::VarError
        let source = read_to_string(file)?; // return std::io::Error
        Ok(source)
    }

    let html = render().unwrap();
    println!("{}", html);
}

#[test]
#[should_panic(expected = "Environment variable not found")]
fn it_generic_error_from_trait() {
    use std::fs::read_to_string;

    fn render() -> Result<String, MyError> {
        let file = std::env::var("MARKDOWN")?;
        let source = read_to_string(file)?;
        Ok(source)
    }

    #[derive(Debug)]
    enum MyError {
        EnvironmentVariableNotFound,
        IOError(std::io::Error),
    }

    // 有为自定义错误类型实现 Error 特征后，才能转换成相应的特征对象
    impl std::error::Error for MyError {}

    impl From<std::env::VarError> for MyError {
        fn from(_: std::env::VarError) -> Self {
            Self::EnvironmentVariableNotFound
        }
    }

    impl From<std::io::Error> for MyError {
        fn from(err: std::io::Error) -> Self {
            Self::IOError(err)
        }
    }

    impl std::fmt::Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::EnvironmentVariableNotFound => write!(f, "Environment variable not found"),
                Self::IOError(err) => write!(f, "IO Error: {}", err.to_string()),
            }
        }
    }

    let content = match render() {
        Ok(content) => content,
        Err(err) => panic!("render failed: {}", err),
    };
    println!("render content: {}", content);
}

#[test]
#[should_panic(expected = "Environment variable not found")]
fn it_generic_error_thiserror() {
    use std::fs::read_to_string;

    #[derive(thiserror::Error, Debug)]
    enum MyError {
        #[error("Environment variable not found")]
        EnvironmentVariableNotFound(#[from] std::env::VarError),
        #[error(transparent)]
        IOError(#[from] std::io::Error),
    }

    fn render() -> Result<String, MyError> {
        let file = std::env::var("RUST")?;
        let content = read_to_string(file)?;
        Ok(content)
    }

    let content = match render() {
        Ok(content) => content,
        Err(err) => panic!("render failed: {}", err),
    };
    println!("render content: {}", content);
}

//
// Unsafe
//

#[test]
fn it_unsafe_get_ptr_from_ref() {
    // 基于引用创建裸指针
    let mut num = 5;
    let p1 = &num as *const i32;
    let p2 = &mut num as *mut i32;

    // 创建裸指针是安全的行为，而解引用裸指针才是不安全的行为
    unsafe {
        println!("number is {}", *p1);
        *p2 = *p1 + 1;
        println!("*p1={}, *p2={}", *p1, *p2);
    }
    println!();

    let a = 1;
    let b: *const i32 = &a as *const i32; // 使用 as 显式的转换
    let c: *const i32 = &a; // 隐式转换
    unsafe {
        println!("b={}, c={}", *b, *c);
    }
}

#[test]
fn it_unsafe_get_ptr_from_box() {
    let a = Box::new(10);
    let b: *const i32 = &*a;
    let c: *const i32 = Box::into_raw(a);
    unsafe {
        println!("b={}, c={}", *b, *c);
    }
}

#[test]
fn it_unsafe_get_ptr_from_addr() {
    use std::{slice::from_raw_parts, str::from_utf8_unchecked};

    // 获取字符串的内存地址和长度
    fn get_memory_location() -> (usize, usize) {
        let s = "hello world";
        let p = s.as_ptr() as usize;
        let len = s.len();
        (p, len)
    }

    // 在指定的内存地址读取字符串
    fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
        unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
    }

    let (pointer, length) = get_memory_location();
    let s = get_str_at_location(pointer, length);
    println!("the {} bytes at 0x{:X} stored: {}", length, pointer, s)
}

#[test]
fn it_unsafe_code_block() {
    use std::slice;

    // 虽然 split_at_mut 使用了 unsafe, 但我们无需将其声明为 unsafe fn
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr(); // 返回指向 slice 首地址的裸指针 *mut i32
        assert!(mid <= len); // 保证 unsafe 中使用的裸指针 ptr 和 ptr.add(mid) 是合法

        // i32 类型每个元素都占用了 4 个字节
        // 这里使用 ptr.add(mid) 代替 ptr + 4 * mid
        unsafe {
            (
                // 通过指针和长度来创建一个新的切片，该切片的初始地址是 ptr, 长度为 mid
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..]; // Vec[T] => &[T]
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    println!("a: {:?}", a);
    println!("b: {:?}", b);
}

#[test]
fn it_unsafe_ffi_for_c() {
    // 调用 C 标准库中的 abs 函数
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[test]
fn it_unsafe_inline_asm() {
    // 内联汇编：在 Rust 代码中嵌入汇编代码
    use std::arch::asm;

    // 将 5 赋给 u64 类型的变量 x
    let x: u64;
    unsafe {
        asm!("mov {}, 5", out(reg) x);
    }
    assert_eq!(x, 5);

    // o = i + 5
    let i: u64 = 3;
    let o: u64;
    unsafe { asm!("mov {0}, {1}", "add {0}, 5", out(reg) o, in(reg) i) }
    assert_eq!(o, 8);

    let x: u64 = 3;
    let y: u64;
    unsafe { asm!("add {0}, 5", inout(reg) x => y) }
    assert_eq!(y, 8);
}
