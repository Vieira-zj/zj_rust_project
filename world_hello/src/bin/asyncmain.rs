use world_hello::webserver::appv3 as webapp;

#[async_std::main]
async fn main() {
    // test:
    // curl http://127.0.0.1:7878/
    // curl http://127.0.0.1:7878/sleep
    run_app_async_websrv(false).await;
    println!("async main done");
}

async fn run_app_async_websrv(is_run: bool) {
    if is_run {
        webapp::tcp_srv().await;
    }
}
