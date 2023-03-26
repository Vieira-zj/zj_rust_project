use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

// 单线程 web server

pub fn tcp_srv() {
    let host = "127.0.0.1:7878";
    println!("http serve at: {host}");
    let listener = TcpListener::bind(host).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = if req_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let mut file_path = String::from("/tmp/test/");
    file_path.push_str(filename);
    let contents = fs::read_to_string(file_path).unwrap();
    let length = contents.len();

    let resp = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(resp.as_bytes()).unwrap();
}

#[allow(dead_code)]
fn handle_connection_deprecated(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("request: {:#?}", http_request);

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("/tmp/test/hello.html").unwrap();
    let length = contents.len();

    let resp = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(resp.as_bytes()).unwrap();
}

/*
hello.html

<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
  </body>
</html>
*/

/*
404.html

<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>你好!</title>
  </head>
  <body>
    <h1>很抱歉!</h1>
    <p>由于运维删库跑路，我们的数据全部丢失，总监也已经准备跑路，88</p>
  </body>
</html>
*/
