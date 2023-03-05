//
// 异步编程
// https://course.rs/async-rust/async/intro.html
//

#[test]
fn it_async_hello_world() {
    use futures::executor::block_on;

    async fn hello_world() {
        hello_cat().await;
        println!("hello, world!");
    }

    async fn hello_cat() {
        println!("hello, kitty!");
    }

    let future = hello_world();
    block_on(future);
}

#[test]
fn it_selfref_sample() {
    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
    }

    impl Test {
        fn new(txt: &str) -> Self {
            Test {
                a: String::from(txt),
                b: std::ptr::null(),
            }
        }

        fn init(&mut self) {
            let self_ref: *const String = &self.a;
            self.b = self_ref;
        }

        fn a(&self) -> &str {
            &self.a
        }

        fn b(&self) -> &String {
            assert!(
                !self.b.is_null(),
                "Test::b called without Test::init being called first"
            );
            unsafe { &(*self.b) }
        }
    }

    let mut test1 = Test::new("test1");
    test1.init();
    let mut test2 = Test::new("test2");
    test2.init();
    println!("a: {}, b: {}", test1.a(), test1.b());
    println!("a: {}, b: {}", test2.a(), test2.b());

    std::mem::swap(&mut test1, &mut test2);
    println!("a: {}, b: {}", test1.a(), test1.b());
    println!("a: {}, b: {}", test2.a(), test2.b());
}

#[test]
fn it_selfref_pin_to_stack() {
    // TODO:
}

#[test]
fn it_selfref_pin_to_heap() {
    // TODO:
}

//
// Exp
//

#[test]
fn it_iterator_slice() {
    fn largest_by_ref(values: &[i32]) -> &i32 {
        let mut largest = &values[0];
        for val in values {
            if val > largest {
                largest = val;
            }
        }
        return largest;
    }

    fn largest_by_copy(values: &[i32]) -> i32 {
        let mut largest = values[0];
        for &val in values {
            if val > largest {
                largest = val;
            }
        }
        largest
    }

    let v = [1, 2, 3];
    let result = largest_by_ref(&v);
    println!("largest: {}", result);

    let result = largest_by_copy(&v);
    println!("largest: {}", result);
}

#[test]
fn it_mut_borrow() {
    fn add_item(data: &mut Vec<i32>) {
        data.push(6);
    }

    let mut data = vec![1, 2, 3];
    data.push(4);
    {
        let bow = &mut data;
        bow.push(5);
    }
    add_item(&mut data);

    println!("{:?}", data);
}

#[test]
fn it_return_fn_local_str() {
    fn get_str<'a>() -> &'a str {
        // error
        // let s = String::from("hello");
        // return s.as_str();

        // ok
        let s = "hello";
        return s;
    }

    println!("{}", get_str());
}
