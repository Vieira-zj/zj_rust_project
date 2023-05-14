use mini_redis::{client, Result};

//
// mini redis server
//
// install:
// $ cargo install mini-redis
// start:
// $ mini-redis-server
// check:
// $ mini-redis-cli set foo 1
// $ mini-redis-cli get foo
//
//

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello", "world".into()).await?;

    let result = client.get("hello").await?;
    println!("get results: {:?}", result);
    Ok(())
}
