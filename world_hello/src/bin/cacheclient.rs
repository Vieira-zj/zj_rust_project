use bytes::Bytes;
use mini_redis::{client, Result};
use tokio::sync::mpsc;
use tokio::sync::oneshot;

// mini redis client

#[tokio::main]
async fn main() {
    run_client_by_queue().await;
    println!("mini cache client done")
}

// Client by Queue

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

async fn run_client_by_queue() {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    // 从 channel 获取 cmd 执行
    let manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    // 往 oneshot 中发送消息时，并没有使用 .await, 原因是该发送操作要么直接成功、要么失败，并不需要等待
                    // 忽略错误
                    let _ = resp.send(res);
                }
                Command::Set { key, val, resp } => {
                    let res = client.set(&key, val).await;
                    // 忽略错误
                    let _ = resp.send(res);
                }
            }
        }
    });

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        };
        // 发送 GET 请求
        tx.send(cmd).await.unwrap();
        // 等待回复
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };
        // 发送 SET 请求
        tx2.send(cmd).await.unwrap();
        // 等待回复
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}

// Client helloworld

#[allow(dead_code)]
async fn run_client_helloworld() -> Result<()> {
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
    async fn test_run_client_helloworld() {
        run_client_helloworld().await.unwrap();
        println!("test mini cache helloworld done")
    }
}
