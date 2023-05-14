use async_std::io::{Read, Write};
use async_std::net::TcpListener;
use async_std::task;
use futures::stream::StreamExt;
use futures::{AsyncReadExt, AsyncWriteExt};
use std::marker::Unpin;
use std::{fs, time::Duration};

// 异步 web server
// refer: https://course.rs/advance/async/web-server.html

pub async fn tcp_srv() {
    let host = "127.0.0.1:7878";
    println!("http serve at: {host}");
    let listener = TcpListener::bind(host).await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();
            handle_connection(stream).await;
        })
        .await;
}

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buf = [0; 1024];
    stream.read(&mut buf).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buf.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buf.starts_with(sleep) {
        task::sleep(Duration::from_secs(3)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let mut file_path = String::from("/tmp/test/");
    file_path.push_str(filename);

    let content = fs::read_to_string(file_path).unwrap();
    let resp = format!("{status_line}{content}");
    stream.write_all(resp.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::webserver::mock_stream::MockTcpStream;

    #[async_std::test]
    async fn test_handle_connection() {
        let input_bytes = b"GET / HTTP/1.1\r\n";
        let mut contents = vec![0u8; 1024];
        contents[..input_bytes.len()].clone_from_slice(input_bytes);
        let mut stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };

        handle_connection(&mut stream).await;
        let mut buf = [0u8; 1024];
        stream.read(&mut buf).await.unwrap();

        let file_path = "/tmp/test/hello.html";
        let expected_body = fs::read_to_string(file_path).unwrap();
        let expected_resp = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_body);
        assert!(stream.write_data.starts_with(expected_resp.as_bytes()));
        // println!("resp: {}", expected_resp);
    }
}
