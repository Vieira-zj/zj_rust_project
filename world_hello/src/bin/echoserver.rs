use tokio::io;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let addr = "127.0.0.1:6142";
    let listener = TcpListener::bind(addr).await?;
    println!("listen at :{}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let (mut r, mut w) = io::split(socket);
            if io::copy(&mut r, &mut w).await.is_err() {
                eprintln!("failed to copy");
            }
        });
    }
}

#[cfg(test)]
mod tests {

    use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    async fn echo_client() -> io::Result<()> {
        let socket = TcpStream::connect("127.0.0.1:6142").await?;
        let (mut r, mut w) = io::split(socket);

        tokio::spawn(async move {
            w.write_all(b"hello\r\n").await?;
            w.write_all(b"world\r\n").await?;
            Ok::<(), io::Error>(())
        });

        let mut buf = vec![0; 128];
        loop {
            let n = r.read(&mut buf).await?;
            if n == 0 {
                break;
            }

            println!("got {:?}", &buf[..n]);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_echo_server() {
        echo_client().await.unwrap();
        println!("test echo server done")
    }
}
