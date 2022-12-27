//
// 多线程
// https://course.rs/advance/concurrency-with-threads/intro.html
//

use std::thread;
use std::time::Duration;

#[test]
fn it_create_and_wait_thread_01() {
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
fn it_create_and_wait_thread_02() {
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
    println!("barrier test done");
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

    // 尽管子线程中修改为了 3, 我们在这里依然拥有main线程中的局部值 2
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

//
// Channel: send and receive message
//

#[test]
fn it_mpsc_send_and_rec_msg() {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    println!("receive {}", rx.recv().unwrap());
}

#[test]
fn it_mpsc_send_and_try_rec_msg() {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(1).unwrap();
    }); // 发送者 tx 被 drop

    println!("receive {:?}", rx.try_recv());
    thread::sleep(Duration::from_millis(100));
    println!("receive {:?}", rx.try_recv());
    thread::sleep(Duration::from_millis(100));
    println!("receive {:?}", rx.try_recv());
}

#[test]
fn it_mpsc_rec_msg_in_for() {
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
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // 'for rx' exit when tx is dropped
    for rec in rx {
        println!("received: {}", rec);
    }
    println!("main exit");
}

#[test]
fn it_mpsc_multiple_send() {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        tx.send(String::from("hi from raw tx")).unwrap();
    });
    thread::spawn(move || {
        tx1.send(String::from("hi from clone tx")).unwrap();
    });

    for rec in rx {
        println!("received: {}", rec);
    }
    println!("main exit");
}

#[test]
fn it_mpsc_send_and_rec_enum() {
    use std::sync::mpsc::{self, Receiver, Sender};

    enum Fruit {
        Apple(u8),
        Orange(String),
    }

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

//
// 锁、Condvar 和信号量
//
