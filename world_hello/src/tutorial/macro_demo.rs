// 自定义 derive 过程宏

pub trait HelloMacro {
    fn hello_macro();
}

pub fn custom_macro_sample() {
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Sunfei;

    #[derive(HelloMacro)]
    struct Sunface;

    Sunfei::hello_macro();
    Sunface::hello_macro();
}
