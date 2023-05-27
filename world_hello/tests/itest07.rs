//
// Async Runtime: Tokio
//

#[tokio::test]
async fn it_async_use_mutex() {
    // 在 .await 执行期间，任务可能会在线程间转移
    use std::sync::Mutex;

    async fn do_something_async() {
        let handler = tokio::spawn(async {
            println!("do_something_async");
        });
        handler.await.unwrap();
    }

    async fn increment_and_do_stuff(mux: &Mutex<i32>) {
        {
            let mut lock = mux.lock().unwrap();
            *lock += 1;
        } // lock 在这里超出作用域被释放
        do_something_async().await;
    }

    let mux = Mutex::new(0);
    for _ in 1..4 {
        increment_and_do_stuff(&mux).await;
    }

    let result = mux.lock().unwrap();
    println!("result: {}", result);
}

#[tokio::test]
async fn it_async_use_channel() {
    // 使用 channel 传递消息
    use tokio::sync::mpsc;

    let (tx, mut rx) = mpsc::channel(8);
    let tx2 = tx.clone();

    tokio::spawn(async move {
        tx.send("sending from first handle").await.unwrap();
    });
    tokio::spawn(async move {
        tx2.send("sending from second handle").await.unwrap();
    });

    while let Some(msg) = rx.recv().await {
        println!("got {}", msg);
    }
    println!("async demo done");
}

#[tokio::test]
async fn it_tokio_io_read() {
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;

    {
        let mut f = File::open("/tmp/test/test.txt").await.unwrap();
        let mut buf = [0; 3];

        let n = f.read(&mut buf[..]).await.unwrap();
        println!("read {} bytes: {:?}", n, &buf[..n]);
    }

    {
        let mut f = File::open("/tmp/test/test.txt").await.unwrap();
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).await.unwrap();
        println!("read {} bytes: {:?}", buf.len(), buf);
    }
}

#[tokio::test]
async fn it_tokio_io_write() {
    use tokio::fs::File;
    use tokio::io::AsyncWriteExt;

    {
        let mut f = File::create("/tmp/test/test1.txt").await.unwrap();
        let n = f.write(b"some bytes").await.unwrap();
        println!("write the 1st {} bytes of 'some bytes'", n);
    }

    {
        let mut f = File::create("/tmp/test/test2.txt").await.unwrap();
        f.write_all(b"some more bytes").await.unwrap();
        println!("write bytes 'some more bytes'");
    }
}

#[tokio::test]
async fn it_tokio_io_copy() {
    use tokio::fs::File;
    use tokio::io;

    // 字节数组 &[u8] 实现了 AsyncRead
    let mut reader: &[u8] = b"hello";
    let mut f = File::create("/tmp/test/test.txt").await.unwrap();
    let n = io::copy(&mut reader, &mut f).await.unwrap();
    println!("copied {} bytes", n);
}

#[tokio::test]
async fn it_tokio_select() {
    use tokio::sync::oneshot;

    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async move {
        let _ = tx1.send("one");
    });
    tokio::spawn(async move {
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }
    println!("tokio select demo done");
}

#[tokio::test]
async fn it_tokio_select_with_cancel() {
    use std::time::Duration;
    use tokio::{sync::oneshot, time};

    async fn some_operation() -> String {
        println!("some_operation start");
        time::sleep(Duration::from_secs(3)).await;
        println!("some_operation end");
        String::from("some operation")
    }

    let (mut tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async move {
        tokio::select! {
            val = some_operation() => {
                let _ = tx1.send(val);
            }
            _ = tx1.closed() => {
                // 收到了发送端发来的关闭信号
                // select 退出，此时正在进行的 some_operation() 任务会被取消
                println!("tx1 is released");
            }
        }
    });
    tokio::spawn(async move {
        let _ = tx2.send("two");
    });

    tokio::select! {
        val = rx1 => {
            println!("rx1 completed first with {:?}", val);
        }
        val = rx2 => {
            println!("rx2 completed first with {:?}", val);
        }
    }

    time::sleep(Duration::from_secs(1)).await;
    println!("tokio select demo done");
}

#[tokio::test]
async fn it_tokio_select_with_else() {
    use tokio::sync::oneshot;

    fn get_value() -> Option<()> {
        return None;
    }

    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async move {
        let _ = tx1.send(get_value());
    });
    tokio::spawn(async move {
        let _ = tx2.send(get_value());
    });

    // 模式匹配，若之前的分支都无法被匹配，那 else 分支将被执行
    tokio::select! {
        Some(v) = async { rx1.await.unwrap() } => {
            println!("Got {:?} from rx1", v);
        }
        Some(v) = async { rx2.await.unwrap() } => {
            println!("Got {:?} from rx2", v);
        }
        else => {
            println!("both channels closed");
        }
    }

    println!("tokio select demo done");
}

#[tokio::test]
async fn it_tokio_select_in_loop() {
    // TODO:
}

//
// Exp
//

#[test]
fn it_slice_and_vec() {
    // 读取使用 &[T] 而不是 &Vec<T>
    fn find_number(nums: &[i32], dst: i32) -> Option<&i32> {
        println!("numbers: {:?}", nums);
        nums.iter().find(|&&x| x == dst)
    }

    let mut nums = vec![1, 2, 3];
    nums.push(10);

    match find_number(&nums, 11) {
        Some(value) => println!("find item: {}", value),
        None => println!("item not found"),
    }
}

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

    let v = [1, 4, 5, 2, 3];
    let result = largest_by_ref(&v);
    println!("largest: {}", result);

    let result = largest_by_copy(&v);
    println!("largest: {}", result);
}

#[test]
fn it_mut_borrow_01() {
    let mut x = 1;
    let y = &x;
    println!("{}, {}", x, y);

    let z = &mut x;
    // error: cannot assign to "x" because it is borrowed
    // x = 2;
    // println!("{}", x);
    *z = 3;
    println!("{}", z);

    let s = "hello";
    let sub = &s[..3];
    println!("{}, {}", s, sub);

    let mut str = String::new();
    // 这里 push_str 需要可变借用 &mut self
    str.push_str("hello");
    println!("string: {}", str);
}

#[test]
fn it_mut_borrow_02() {
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
        // String 的作用域为 get_str 函数，而字符串字面量 "hello" 的生命周期是 'static
        // error
        // let s = String::from("hello");
        // return s.as_str();

        // ok
        let s = "hello";
        return s;
    }

    println!("{}", get_str());
}
