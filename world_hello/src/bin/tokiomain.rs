#[tokio::main]
async fn main() {
    run_stdin_and_stdout(false).await;

    println!("async tokio main done");
}

async fn run_stdin_and_stdout(is_run: bool) {
    if is_run {
        match interactive_stdin_and_stdout().await {
            Ok(_) => println!("ineractive exit"),
            Err(err) => println!("ineractive error: {:?}", err),
        }
    }
}

// sample: stdin and stdout

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

async fn interactive_stdin_and_stdout() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout
        .write_all("pls input, q for quit\n".as_bytes())
        .await?;

    let mut stdin = io::stdin();
    loop {
        let mut buf = vec![0u8; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        let input = String::from_utf8_lossy(&buf).into_owned();
        let input = input.trim_end_matches("\n");
        if input == "q" {
            break;
        }
        println!("the input is: {:?}", input);
    }

    Ok(())
}
