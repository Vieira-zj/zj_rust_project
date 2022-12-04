//
// 全局变量
// https://course.rs/advance/global-variable.html
//

#[test]
fn it_id_generator_by_atomic() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
    const MAX_ID: usize = usize::MAX / 2;

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
fn it_cacher_by_lazy_static() {
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

    println!("the value for 0 is: {}", HASHMAP.get(&0).unwrap());
    println!("the value for 1 is: {}", HASHMAP.get(&1).unwrap());
}

//
// Option, Result API
//

#[test]
fn it_func_or_and() {
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
fn it_func_or_else_and_then() {
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
fn it_func_filter() {
    let s1 = Some(3);
    let s2 = Some(6);
    let n = None;

    let fn_is_even = |x: &i8| x % 2 == 0;
    assert_eq!(s1.filter(fn_is_even), n);
    assert_eq!(s2.filter(fn_is_even), s2);
    assert_eq!(n.filter(fn_is_even), n);
}

#[test]
fn it_func_map() {
    // Option
    let s1 = Some("abcde");
    let s2 = Some(5);

    let n1: Option<&str> = None;
    let n2: Option<usize> = None;

    let fn_character_count = |s: &str| s.chars().count();

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
fn it_func_map_err() {
    let o1: Result<&str, &str> = Ok("abcde");
    let o2: Result<&str, isize> = Ok("abcde");

    let e1: Result<&str, &str> = Err("404");
    let e2: Result<&str, isize> = Err(404);

    let fn_character_count = |s: &str| -> isize { s.parse().unwrap() };

    assert_eq!(o1.map_err(fn_character_count), o2);
    assert_eq!(e1.map_err(fn_character_count), e2);
}

#[test]
fn it_func_map_or() {
    const V_DEFAULT: u32 = 1;
    let ok: Result<u32, ()> = Ok(10);
    let n: Option<u32> = None;
    let fn_closure = |v: u32| v + 2;

    assert_eq!(ok.map_or(V_DEFAULT, fn_closure), 12);
    assert_eq!(n.map_or(V_DEFAULT, fn_closure), 1);
}

#[test]
fn it_func_map_or_else() {
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
fn it_func_ok_or() {
    const ERR_DEFAULT: &str = "error message";

    let s = Some("abcde");
    let n: Option<&str> = None;

    let ok: Result<&str, &str> = Ok("abcde");
    let e: Result<&str, &str> = Err(ERR_DEFAULT);

    assert_eq!(s.ok_or(ERR_DEFAULT), ok);
    assert_eq!(n.ok_or(ERR_DEFAULT), e);
}

#[test]
fn it_func_ok_or_else() {
    let s = Some("abcde");
    let n: Option<&str> = None;
    let fn_err_message = || "error message";

    let ok: Result<&str, &str> = Ok("abcde");
    let e: Result<&str, &str> = Err("error message");

    assert_eq!(s.ok_or_else(fn_err_message), ok);
    assert_eq!(n.ok_or_else(fn_err_message), e);
}

//
// 错误处理
//
