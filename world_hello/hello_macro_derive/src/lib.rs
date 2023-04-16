extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput};

// refer: https://course.rs/advance/macro.html

// 特征 HelloMacro 将用户类型和过程宏联系在一起
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // 基于 input 构建 AST 语法树
    let ast = syn::parse(input).unwrap();
    // 构建特征实现代码
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify! 将一个表达式在编译期转换成一个字符串字面值，该字面量具有 'static 生命周期
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
