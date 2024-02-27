use std::path::Path;
use std::sync::Arc;
use tokio::{fs::File, io::AsyncWriteExt};
use tokio_stream::StreamExt;

pub fn run() {
    // url is used in thread, so alone with Arc
    let urls = vec![
        Arc::new("http://api.example.com/data1"),
        Arc::new("http://api.example.com/data2"),
    ];

    let download_futures: Vec<_> = urls
        .iter()
        .map(|url| tokio::spawn(download_file(&url.clone())))
        .collect();

    let results = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(futures::future::join_all(download_futures));

    for res in results {
        if let Err(e) = res {
            println!("download error: {}", e);
        }
    }
}

// note: the returned error must be Send + Sync when run in tokio::spawn
async fn download_file(url: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let resp = reqwest::get(url).await?;

    let path = Path::new("/tmp/downloads").join(url.split("/").last().unwrap());
    let mut file = File::create(&path).await?;

    if resp.status().is_success() {
        let mut stream = resp.bytes_stream();
        while let Some(item) = stream.next().await {
            file.write_all(&item?).await?;
        }
    }

    println!("download success: {}", url);
    Ok(())
}
