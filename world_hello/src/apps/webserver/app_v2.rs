use crate::apps::webserver::pool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

// 多线程 web server
// refer: https://course.rs/advance-practice1/multi-threads.html

pub fn tcp_srv() {
    let host = "127.0.0.1:7878";
    println!("http serve at: {host}");

    let listener = TcpListener::bind(host).unwrap();
    let pool = pool::ThreadPool::new(4);

    // 测试，只接收前两个请求，然后就结束监听，随后 ThreadPool 也将超出作用域并自动触发 drop
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(move || {
            handle_connection(stream);
        });
    }
    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // 由于 match 不会像方法那样自动做引用或者解引用，因此我们需要显式调用 match &request_line[..], 来获取所需的 &str 类型
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let mut file_path = String::from("/tmp/test/");
    file_path.push_str(filename);
    let contents = fs::read_to_string(file_path).unwrap();
    let length = contents.len();

    let resp = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}
