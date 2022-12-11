//
// 多线程
// https://course.rs/advance/concurrency-with-threads/intro.html
//

use std::thread;
use std::time::Duration;

#[test]
fn it_create_and_wait_thread() {
    // main 线程一旦结束，则程序随之结束，同时各个子线程也将被强行终止
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(100));
        }
    });
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(100));
    }
}

#[test]
fn it_move_in_thread() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
    println!("test done");
}

#[test]
fn it_barrier_for_threads() {
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
    println!("test done");
}

#[test]
fn it_thread_local() {
    use std::cell::RefCell;

    // FOO 使用 static 声明为生命周期为 'static 的静态变量
    thread_local! {static FOO: RefCell<u32> = RefCell::new(1)};

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    let t = thread::spawn(move || {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1);
            *f.borrow_mut() = 3;
        });
    });
    t.join().unwrap();

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2);
    });
}

#[test]
fn it_thread_lock_and_notify() {
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
fn it_thread_sync_once() {
    use std::sync::Once;

    static mut VAL: usize = 0;
    static INIT: Once = Once::new();

    let handle1 = thread::spawn(|| {
        INIT.call_once(|| unsafe {
            println!("set value 1");
            VAL = 1;
        });
    });

    let handle2 = thread::spawn(|| {
        INIT.call_once(|| unsafe {
            println!("set value 2");
            VAL = 2;
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("{}", unsafe { VAL });
}
