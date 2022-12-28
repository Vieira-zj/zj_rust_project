//
// 异步编程
// https://course.rs/async-rust/async/intro.html
//

#[test]
fn it_async_hello_world() {
    use futures::executor::block_on;

    async fn hello_world() {
        hello_cat().await;
        println!("hello, world!");
    }

    async fn hello_cat() {
        println!("hello, kitty!");
    }

    let future = hello_world();
    block_on(future);
}
