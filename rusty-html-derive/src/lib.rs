#![feature(proc_macro_span)]
#![feature(proc_macro_span_shrink)]

#![feature(extend_one)]

mod parser;
use parser::Parser;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn html(input : TokenStream) -> TokenStream {
    match Parser::new(input) {
        Some(parser) => {
            quote! {
                {
                    let mut html = String::new();
                    #(#parser)*
                    html
                }
            }.into()
        },
        None => TokenStream::new()
    }  
}