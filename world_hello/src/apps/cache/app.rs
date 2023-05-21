use crate::apps::cache::process;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

// mini redis cache server

#[allow(dead_code)]
pub async fn srv() {
    let addr = "127.0.0.1:6379";
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening: {}", addr);

    let db: process::Db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();

        println!("Accepted");
        let t = tokio::spawn(async move {
            process::run(socket, db).await;
        });
        t.await.unwrap();
    }
}
