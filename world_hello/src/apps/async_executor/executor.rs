use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::Context,
    },
};

/// `Spawner` 负责创建新的 `Future` 然后将它发送到任务通道中
#[derive(Clone)]
pub struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        println!("Spawner.spawn: add task");
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("task queue is full")
    }
}

/// 一个 Future, 它可以调度自己（将自己放入任务通道中），然后等待执行器去 `poll`
struct Task {
    /// 进行中的 Future, 在未来的某个时间点会被完成
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    /// 可以将该任务自身放回到任务通道中，等待执行器的 poll
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        println!("Task.wake_by_ref: trigger by wake()");
        // 通过发送任务到任务管道的方式来实现 wake, 这样 wake 后，任务就能被执行器 poll
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("task queue is full")
    }
}

/// 任务执行器，负责从通道中接收任务然后执行
pub struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    pub fn run(&self) {
        let tag = "Executor.run";
        // while let Ok(task) = self.ready_queue.recv() {
        loop {
            match self.ready_queue.recv() {
                Ok(task) => {
                    println!("{tag}: get task from queue");
                    let mut future_slot = task.future.lock().unwrap();
                    if let Some(mut future) = future_slot.take() {
                        // 获取一个 future, 若它还没有完成（仍然是 Some, 不是 None），则对它进行一次 poll 并尝试完成它
                        println!("{tag}: init ctx, and invoke future poll()");
                        let waker = waker_ref(&task);
                        let context = &mut Context::from_waker(&waker);
                        // BoxFuture<T> 是 Pin<Box<dyn Future<Output = T> + Send + 'static>> 的类型别名
                        // 通过调用 as_mut 方法，可以将上面的类型转换成 Pin<&mut dyn Future + Send + 'static>
                        if future.as_mut().poll(context).is_pending() {
                            // Future 还没执行完，因此将它放回任务中，等待下次被 poll
                            *future_slot = Some(future);
                        }
                    }
                }
                Err(err) => {
                    println!("{tag}: receive err from queue: {:?}", err);
                    println!("{tag}: out of loop, and exit");
                    break;
                }
            }
        }
    }
}

pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}
