use mini_redis::{client, Result};
use world_hello::apps::cache::app;

/*
mini redis server

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

#[allow(dead_code)]
async fn client_helloworld() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;
    println!("get results: {:?}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_helloworld() {
        client_helloworld().await.unwrap();
        println!("test mini cache helloworld done")
    }
}
