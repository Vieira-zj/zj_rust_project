use hello_macro_derive::HelloMacro;

// 自定义 derive 过程宏

pub trait HelloMacro {
    fn hello_macro();
}

#[derive(HelloMacro)]
struct Sunfei;

#[derive(HelloMacro)]
struct Sunface;

pub fn custom_macro(is_run: bool) {
    if is_run {
        Sunfei::hello_macro();
        Sunface::hello_macro();
    }
}
