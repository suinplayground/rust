// extern crate proc_macro;
//
// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, LitStr};
//
// #[proc_macro]
// pub fn alphanumeric_only(input: TokenStream) -> TokenStream {
//     let input_string = parse_macro_input!(input as LitStr);
//     let string_value = input_string.value();
//     if !string_value.chars().all(|c| c.is_alphanumeric()) {
//         panic!("Only alphanumeric characters are allowed!")
//     }
//     format!()
//     TokenStream::from(quote! {
//         #input_string
//     })
// }
//
//
