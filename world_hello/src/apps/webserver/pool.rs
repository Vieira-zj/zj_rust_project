use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

// thread pool

type Job = Box<dyn FnOnce() + Send + 'static>; // 类型为特征对象

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        // Arc 允许多个 Worker 同时持有 receiver（安全共享）
        // 而 Mutex 可以确保一次只有一个 Worker 能从 receiver 接收消息（互斥使用）
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        // 参考 thread::spawn 函数签名
        // FnOnce:  闭包作为任务只需被线程执行一次即可
        // Send:    闭包需要从一个线程传递到另一个线程
        // 'static: 我们并不知道线程需要多久时间来执行该任务
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 关闭 sender 后，将关闭对应的 channel, 意味着不会再有任何消息被发送。随后，所有的处于无限 loop 的接收端将收到一个错误
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

// worker

struct Worker {
    id: usize,
    // 对于 Option 类型，可以使用 take 方法拿走内部值的所有权
    // JoinHandle 的 T 是传入的闭包任务所返回的，我们的任务无需任何返回，因此 T 直接使用 () 即可
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            // Mutex 结构体没有提供显式的 unlock, 要依赖作用域结束后的 drop 来自动释放
            // 由于这里使用了 let, 右边的任何临时变量会在 let 语句结束后立即被 drop, 因此锁会自动释放
            // recv 的调用过程是阻塞的
            let message = receiver.lock().unwrap().recv();
            // 这里锁已经释放
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down");
                    break;
                }
            }
        });

        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}
