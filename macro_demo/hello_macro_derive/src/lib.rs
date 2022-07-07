extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input : TokenStream) -> TokenStream{
    println!("TokenStream:{}",input);
    // 基于 input 构建 AST 语法树
    let ast = syn::parse(input).unwrap();
    // 构建特征实现代码
    let r = impl_hello_macro(&ast);
    println!("result:{}",r);
    r
}
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}