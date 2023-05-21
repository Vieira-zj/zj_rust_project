use world_hello::apps::cache::app;

/*
mini redis server
refer: https://course.rs/advance-practice/intro.html

install:
$ cargo install mini-redis
start:
$ mini-redis-server
check:
$ mini-redis-cli set foo 1
$ mini-redis-cli get foo
*/

// #[tokio::main] 宏在将 async fn main 隐式的转换为 fn main 的同时还对整个异步运行时进行了初始化
#[tokio::main]
async fn main() {
    app::srv().await;
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_task_spawn_with_move() {
        use tokio::task;

        let v = vec![1, 2, 3];
        let handler = task::spawn(async move {
            println!("here is a vector: {:?}", v);
        });
        handler.await.unwrap();
    }
}
