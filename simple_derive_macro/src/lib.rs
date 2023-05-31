extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Simple)]
pub fn simple_derive(input: TokenStream) -> TokenStream {
    // 入力を解析する
    let input = parse_macro_input!(input as DeriveInput);

    // 入力に対して変換を行う（この例では何もしない）
    let name = input.ident;
    let a = input.attrs;

    // 変換した入力をトークンストリームに戻す
    let expanded = quote! {
        impl #name {
            fn simple(&self) {

            }
        }
    };
    println!("Generated code:\n{}", expanded.to_string()); // 生成されたコードを表示

    TokenStream::from(expanded)
}
