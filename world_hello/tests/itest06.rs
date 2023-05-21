//
// 多线程
// https://course.rs/advance/concurrency-with-threads/intro.html
//

use std::thread;
use std::time::Duration;

#[test]
fn it_thread_create_and_wait_01() {
    // main 线程一旦结束，则程序随之结束，同时各个子线程也将被强行终止
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(100));
    }
    handle.join().unwrap();
    println!("test done");
}

#[test]
fn it_thread_create_and_wait_02() {
    // 非 main 线程结束后，各个子线程仍然会继续运行
    let new_thread = thread::spawn(|| {
        println!("thread A creates thread B");
        thread::spawn(|| {
            for i in 0..5 {
                println!("thread B is running at {}", i);
                thread::sleep(Duration::from_millis(200));
            }
        });
        println!("thread A exit");
    });

    new_thread.join().unwrap();
    thread::sleep(Duration::from_secs(3));
    println!("main exit");
}

#[test]
fn it_thread_move_keyword() {
    // 使用 move 关键字拿走 v 的所有权
    // 防止 v 还未在子线程中使用就在主线程中被回收
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
    println!("test done");
}

#[test]
fn it_thread_barrier() {
    use std::sync::{Arc, Barrier};

    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for i in 0..6 {
        let b = barrier.clone();
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 100));
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("barrier test done");
}

#[test]
fn it_thread_local_var() {
    use std::cell::RefCell;

    // FOO 使用 static 声明为生命周期为 'static 的静态变量
    thread_local! {static FOO: RefCell<u32> = RefCell::new(1)};

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    // 每个线程开始时都会拿到线程局部变量 FOO 的初始值
    let t = thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });
    t.join().unwrap();

    // 尽管子线程中修改为了 3, 我们在这里依然拥有main线程中的局部值 2
    FOO.with(|f| {
        let val = *f.borrow();
        assert_eq!(val, 2);
        println!("value: {}", val);
    });
}

#[test]
fn it_thread_lock_and_cond() {
    use std::sync::{Arc, Condvar, Mutex};

    // main 线程首先进入 while 循环，调用 wait 方法挂起等待子线程的通知，并释放了锁 started
    // 子线程获取到锁，并将其修改为 true, 然后调用条件变量的 notify_one 方法来通知主线程继续执行
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (ref lock, ref cvar) = *pair2;
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
    });

    let (ref lock, ref cvar) = *pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("started changed");
}

#[test]
fn it_thread_sync_call_once() {
    use std::sync::Once;

    static mut VAL: usize = 0;
    static INIT: Once = Once::new();

    // 只执行 1 次，如果当前有另一个初始化过程正在运行，线程将阻止该方法被调用
    let handle1 = thread::spawn(|| {
        INIT.call_once(|| {
            println!("set value 1");
            unsafe {
                VAL = 1;
            }
        });
    });

    let handle2 = thread::spawn(|| {
        INIT.call_once(|| {
            println!("set value 2");
            unsafe {
                VAL = 2;
            }
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("{}", unsafe { VAL });
}

//
// Channel: send and receive message
//
// 关闭通道
// 所有发送者被 drop 或者所有接收者被 drop 后，通道会自动关闭。
//

#[test]
fn it_mpsc_chan_send_and_rec_msg() {
    use std::sync::mpsc;

    // 使用 move 将 tx 的所有权转移到子线程的闭包中
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    // rx.recv() 会阻塞当前线程，直到读取到值，或者通道被关闭
    println!("receive {}", rx.recv().unwrap());
}

#[test]
fn it_mpsc_chan_send_and_try_rec_msg() {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(1).unwrap();
    }); // 发送者 tx 被 drop

    println!("receive {:?}", rx.try_recv()); // receive Err(Empty)
    thread::sleep(Duration::from_millis(100));
    println!("receive {:?}", rx.try_recv()); // receive Ok(1)
    thread::sleep(Duration::from_millis(100));
    println!("receive {:?}", rx.try_recv()); // receive Err(Disconnected)
}

#[test]
fn it_mpsc_chan_rec_msg_in_for() {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            // 传递数据的所有权，不能是引用
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // for 循环阻塞的从 rx 迭代器中接收消息，当子线程运行完成时，发送者 tx 会随之被 drop, 此时 for 循环将被终止
    for rec in rx {
        println!("received: {}", rec);
    }
    println!("main exit");
}

#[test]
fn it_mpsc_chan_multiple_send() {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        tx.send(String::from("hi from raw tx")).unwrap();
    });
    thread::spawn(move || {
        tx1.send(String::from("hi from clone tx")).unwrap();
    });

    // 需要所有的发送者都被 drop 掉后，接收者 rx 才会收到错误，进而跳出 for 循环
    for rec in rx {
        println!("received: {}", rec);
    }
    println!("main exit");
}

#[test]
fn it_mpsc_chan_send_and_rec_enum() {
    use std::sync::mpsc::{self, Receiver, Sender};

    enum Fruit {
        Apple(u8),
        Orange(String),
    }

    // channel 没有缓冲值参数 => 异步通道
    let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();
    tx.send(Fruit::Orange("sweet".to_string())).unwrap();
    tx.send(Fruit::Apple(1)).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => println!("received {} apples", count),
            Fruit::Orange(flavor) => println!("received {} orange", flavor),
        }
    }
}

#[test]
fn it_mpsc_chan_drop() {
    use std::sync::mpsc;
    use std::thread;

    let (send, recv) = mpsc::channel();
    let num_threads = 3;
    for i in 0..num_threads {
        let thread_send = send.clone();
        thread::spawn(move || {
            thread_send.send(i).unwrap();
            println!("thread {} finished", i);
        });
    }

    // send 本身没有被 drop, 会导致 "for in recv" 阻塞
    drop(send);

    for x in recv {
        println!("got: {}", x);
    }
    println!("finished iterating");
}

//
// 锁、Condvar 和信号量
//
// 消息传递类似一个单所有权的系统：一个值同时只能有一个所有者，如果另一个线程需要该值的所有权，需要将所有权通过消息传递进行转移。
// 而共享内存类似于一个多所有权的系统：多个线程可以同时访问同一个值。
//

#[test]
fn it_mutex_lock() {
    use std::sync::Mutex;

    let m = Mutex::new(5);
    {
        // lock() 申请一个锁，该方法会阻塞当前线程，直到获取到锁
        let mut num = m.lock().unwrap();
        *num = 6; // 自动解引用
    } // 锁自动被 drop

    let mut num1 = m.lock().unwrap();
    *num1 = 7;
    drop(num1); // 手动 drop 锁

    println!("m = {:?}", m);
}

#[test]
fn it_mutex_lock_in_thread() {
    use std::sync::{Arc, Mutex};

    // Rc<T> 和 RefCell<T> 的结合，可以实现单线程的内部可变性
    // Arc<T> + Mutex<T> 可以实现多线程的内部可变性
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let cc = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = cc.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("result: {}", *counter.lock().unwrap());
}

#[test]
fn it_mutex_trylock_for_deadlock() {
    // try_lock 会尝试去获取一次锁，如果无法获取会返回一个错误，不会发生阻塞
    use lazy_static::lazy_static;
    use std::sync::{Mutex, MutexGuard};

    lazy_static! {
        static ref MUTEX1: Mutex<i64> = Mutex::new(0);
        static ref MUTEX2: Mutex<i64> = Mutex::new(0);
    }

    let mut children = vec![];
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            if i_thread % 2 == 0 {
                let _guard: MutexGuard<i64> = MUTEX1.lock().unwrap();
                println!(
                    "thread {} get lock for MUTEX1, and try lock MUTEX2",
                    i_thread
                );
                thread::sleep(Duration::from_millis(10));
                let guard = MUTEX2.try_lock();
                println!("thread1 try lock for MUTEX2, result: {:?}", guard);
            } else {
                let _guard = MUTEX2.lock().unwrap();
                println!(
                    "thread {} get lock for MUTEX2, and try lock MUTEX1",
                    i_thread
                );
                thread::sleep(Duration::from_millis(20));
                let guard = MUTEX1.try_lock();
                println!("thread2 try lock for MUTEX1, result: {:?}", guard);
            }
        }));
    }

    for child in children {
        let _ = child.join();
    }
    println!("dead lock not occur")
}

#[test]
fn it_mutex_rwlock() {
    use std::sync::RwLock;

    let lock = RwLock::new(5);
    // 同一时间允许多个读
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // 读锁在此处被 drop

    // 同一时间只允许一个写
    {
        let mut w = lock.write().unwrap();
        *w += 1;
        assert_eq!(*w, 6);
    } // 写锁在此处被 drop
    println!("rwlock test done")
}

#[test]
fn it_sync_condvar() {
    // 实现交替打印输出
    use std::sync::{Arc, Condvar, Mutex};

    let flag = Arc::new(Mutex::new(false));
    let cond = Arc::new(Condvar::new());

    let cflag = flag.clone();
    let ccond = cond.clone();
    let handle = thread::spawn(move || {
        let mut m = { *cflag.lock().unwrap() }; // 代码块中执行，drop 锁
        let mut counter = 0;
        while counter < 3 {
            while !m {
                m = *ccond.wait(cflag.lock().unwrap()).unwrap();
            }
            {
                m = false;
                *cflag.lock().unwrap() = false;
            } // 代码块中执行，drop 锁
            counter += 1;
            println!("inner counter: {}", counter);
        }
    });

    let mut counter = 0;
    loop {
        thread::sleep(Duration::from_secs(1));
        *flag.lock().unwrap() = true;
        counter += 1;
        if counter > 3 {
            break;
        }
        println!("outside counter: {}", counter);
        cond.notify_one();
    }

    handle.join().unwrap();
    println!("flag: {:?}", flag);
}

//
// Atomic 原子操作
//

#[test]
fn it_atomic_global_var() {
    use std::ops::Sub;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::Instant;

    const N_TIMES: u64 = 10000;
    const N_THREADS: usize = 10;

    static R: AtomicU64 = AtomicU64::new(0);

    fn add_n_times(n: u64) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            for _ in 0..n {
                R.fetch_add(1, Ordering::Relaxed);
            }
        })
    }

    let start = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);
    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    for thread in threads {
        thread.join().unwrap();
    }

    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));
    println!("{:?}", Instant::now().sub(start));
}

#[test]
fn it_atomic_var_in_thread() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::{hint, thread};

    // Atomic + Arc
    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = Arc::clone(&spinlock);
    let thread = thread::spawn(move || {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    while spinlock.load(Ordering::SeqCst) != 0 {
        hint::spin_loop();
    }

    if let Err(panic) = thread.join() {
        println!("Thread had an error: {:?}", panic);
    }
}

#[test]
fn it_atomic_var_ordering() {
    use std::sync::atomic::{AtomicBool, Ordering};

    static mut DATA: u64 = 0;
    static READY: AtomicBool = AtomicBool::new(false);

    fn reset() {
        unsafe {
            DATA = 0;
        }
        READY.store(false, Ordering::Relaxed);
    }

    fn producer() -> thread::JoinHandle<()> {
        thread::spawn(move || {
            unsafe {
                DATA = 100;
            }
            READY.store(true, Ordering::Release);
        })
    }

    fn consumer() -> thread::JoinHandle<()> {
        thread::spawn(move || {
            while !READY.load(Ordering::Acquire) {}
            assert_eq!(100, unsafe { DATA });
        })
    }

    for i in 0..10 {
        println!("run producer and consumer at {}", i);
        reset();
        let p = producer();
        let c = consumer();
        p.join().unwrap();
        c.join().unwrap();
        thread::sleep(Duration::from_millis(10));
    }
    println!("test atomic ordering done")
}

//
// 异步编程
// https://course.rs/async-rust/async/intro.html
//

#[test]
fn it_async_run_future() {
    use futures::executor::block_on;

    async fn hello_cat() {
        println!("hello, kitty!");
    }
    async fn hello_world() {
        hello_cat().await;
        println!("hello, world!");
    }

    let future = hello_world();
    block_on(future);
    println!("future done");
}

#[test]
fn it_async_join_futures() {
    use futures::executor::block_on;

    struct Song {
        author: String,
        name: String,
    }

    async fn learn_song() -> Song {
        Song {
            author: "foo".to_string(),
            name: String::from("test_song"),
        }
    }
    async fn sing_song(song: Song) {
        println!("sing song: author={}, name={}", song.author, song.name);
    }
    async fn dance() {
        println!("dance");
    }
    async fn learn_and_sing() {
        let song = learn_song().await;
        sing_song(song).await;
    }

    async fn async_main() {
        let f1 = learn_and_sing();
        let f2 = dance();
        futures::join!(f1, f2); // 并发处理
    }

    block_on(async_main());
    println!("join futures done");
}

#[test]
fn it_async_try_join_futures() {
    use futures::executor::block_on;
    use futures::{future::TryFutureExt, try_join};

    struct Book {
        name: String,
    }
    struct Music {
        name: String,
    }

    async fn get_book() -> Result<Book, ()> {
        Ok(Book {
            name: "test_book".to_string(),
        })
    }
    async fn get_music() -> Result<Music, String> {
        Ok(Music {
            name: "test_music".to_string(),
        })
    }
    async fn get_book_and_music() -> Result<(Book, Music), String> {
        // 注意，传给 try_join! 的所有 Future 都必须拥有相同的错误类型，这里使用 map_err 将错误进行转换
        let book_fut = get_book().map_err(|()| "book not found".to_string());
        let music_fut = get_music();
        try_join!(book_fut, music_fut)
    }

    match block_on(get_book_and_music()) {
        Ok(item) => println!("book={}, music={}", item.0.name, item.1.name),
        Err(err) => println!("{}", err),
    }
    println!("try join futures done");
}

#[test]
fn it_async_select_futures() {
    use futures::executor::block_on;
    use futures::stream::{FusedStream, Stream, StreamExt};
    use futures::{future, select};

    async fn add_two_futures() {
        let mut a_fut = future::ready(4);
        let mut b_fut = future::ready(6);
        let mut total = 0;

        loop {
            select! {
               a = a_fut => total += a,
               b = b_fut => total += b,
               complete => break, // 所有的 Future 完成后执行
               default => panic!(), // not happen
            }
        }

        println!("total={}", total);
        assert_eq!(total, 10);
    }

    #[allow(dead_code)]
    async fn add_two_streams<T>(mut s1: T, mut s2: T) -> u8
    where
        // - Unpin, 由于 select 不会通过拿走所有权的方式使用 Future, 而是通过可变引用的方式去使用，
        // 这样当 select 结束后，该 Future 若没有被完成，它的所有权还可以继续被其它代码使用
        //
        // - FusedFuture 的原因跟上面类似，当 Future 一旦完成后，那 select 就不能再对其进行轮询使用。
        // Fuse 意味着熔断，相当于 Future 一旦完成，再次调用 poll 会直接返回 Poll::Pending
        T: Stream<Item = u8> + FusedStream + Unpin,
    {
        let mut total = 0;
        loop {
            let item = select! {
                x = s1.next() => x,
                x = s2.next() => x,
                complete => break,
            };
            if let Some(next_num) = item {
                total += next_num;
            }
        }

        total
    }

    block_on(add_two_futures());
    println!("select futures done");
}

//
// Pin 和 Unpin
//

#[test]
fn it_pin_selfref_issue() {
    // Test 为一个自引用结构体
    #[derive(Debug)]
    struct Test {
        a: String,
        // 这里 b 是 a 的一个引用，但是我们并没有使用引用类型而是用了裸指针
        // 原因是 Rust 的借用规则不允许我们这样用，因为不符合生命周期的要求
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
    println!("test1: a->{}, b->{}", test1.a(), test1.b());
    println!("test2: a->{}, b->{}", test2.a(), test2.b());

    // 移动数据，b 指针依然指向了旧的地址
    std::mem::swap(&mut test1, &mut test2);
    println!("test1: a->{}, b->{}", test1.a(), test1.b());
    println!("test2: a->{}, b->{}", test2.a(), test2.b());
}

#[test]
fn it_pin_selfref_to_stack() {
    // 将值固定到栈上
    use std::marker::PhantomPinned;
    use std::pin::Pin;

    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
        // 这个标记可以让我们的类型自动实现特征 !Unpin
        _marker: PhantomPinned,
    }

    impl Test {
        fn new(txt: &str) -> Self {
            Test {
                a: String::from(txt),
                b: std::ptr::null(),
                _marker: PhantomPinned,
            }
        }

        fn init(self: Pin<&mut Self>) {
            let self_ptr: *const String = &self.a;
            let this = unsafe { self.get_unchecked_mut() };
            this.b = self_ptr;
        }

        fn a(self: Pin<&Self>) -> &str {
            &(self.get_ref().a)
        }

        fn b(self: Pin<&Self>) -> &String {
            assert!(
                !self.b.is_null(),
                "Test::b called without Test::init being called first"
            );
            unsafe { &*(self.b) }
        }
    }

    let mut test1 = Test::new("test1");
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut()); // 直接调用对象方法

    let mut test2 = Test::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    Test::init(test2.as_mut());

    println!(
        "test1: a->{}, b->{}",
        Test::a(test1.as_ref()),
        Test::b(test1.as_ref())
    );

    // 尝试移动被固定的值，会导致编译错误
    // std::mem::swap(test1.get_mut(), test2.get_mut());
}

#[test]
fn it_pin_selfref_to_heap() {
    // 将一个 !Unpin 类型的值固定到堆上，会给予该值一个稳定的内存地址，它指向的堆中的值在 Pin 后是无法被移动的
    use std::marker::PhantomPinned;
    use std::pin::Pin;

    #[derive(Debug)]
    struct Test {
        a: String,
        b: *const String,
        _marker: PhantomPinned,
    }

    impl Test {
        fn new(txt: &str) -> Pin<Box<Test>> {
            let t = Test {
                a: String::from(txt),
                b: std::ptr::null(),
                _marker: PhantomPinned,
            };
            let mut boxed = Box::pin(t);
            let self_ptr: *const String = &boxed.as_ref().a;
            unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };
            boxed
        }

        fn a(self: Pin<&Self>) -> &str {
            &(self.get_ref().a)
        }

        fn b(self: Pin<&Self>) -> &String {
            unsafe { &*(self.b) }
        }
    }

    let test1 = Test::new("test1");
    let test2 = Test::new("test2");

    println!(
        "test1: a->{}, b->{}",
        test1.as_ref().a(),
        test1.as_ref().b()
    );
    println!(
        "test2: a->{}, b->{}",
        test2.as_ref().a(),
        test2.as_ref().b()
    );
}
