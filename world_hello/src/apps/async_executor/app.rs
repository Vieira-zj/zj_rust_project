use crate::apps::async_executor::{executor, future};
use std::time::Duration;

// 执行流程：
//
// 执行器会管理一批 Future (最外层的 async 函数), 然后通过不停地 poll 推动它们直到完成。
// 最开始，执行器会先 poll 一次 Future, 后面就不会主动去 poll 了，而是等待 Future 通过调用 wake 函数来通知它可以继续，它才会继续去 poll.
// 这种 wake 通知然后 poll 的方式会不断重复，直到 Future 完成。
//

pub fn start() {
    let (executor, spawner) = executor::new_executor_and_spawner();

    spawner.spawn(async {
        println!("future: howdy!");
        future::TimerFuture::new(Duration::new(2, 0)).await;
        println!("future: done!");
    });
    drop(spawner);

    executor.run();
    println!("run async executor demo done");
}
