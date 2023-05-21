use crate::apps::cache::process;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

// 实现一个简单 redis
// refer: https://course.rs/advance-practice/intro.html

#[allow(dead_code)]
pub async fn srv() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");

    let db: process::Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();

        println!("Accepted");
        tokio::spawn(async move {
            process::run(socket, db).await;
        });
    }
}
