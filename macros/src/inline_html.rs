use pest::Parser;
use proc_macro::TokenStream;
use quote::quote;

use crate::build::build_render_function_body;
use crate::grammar::Grammar;
use crate::grammar::Rule;
use crate::helper_macros::err_to_token_stream;

pub(crate) fn build_render_scope(input_stream: TokenStream) -> TokenStream {
    let mut input_stream_inter = input_stream.into_iter();

    let input_span = match (input_stream_inter.next(), input_stream_inter.last()) {
        (Some(first_token), Some(last_token)) => first_token
            .span()
            .join(last_token.span())
            .expect("Procudural Macros can not span multiple files"),
        (Some(first_token), None) => first_token.span(),
        _ => return quote!(String::new()).into(),
    };

    let source_text = input_span
        .source_text()
        .expect("Source test should be real");

    let grammar = err_to_token_stream!(Grammar::parse(Rule::html, &source_text), input_span.into());

    let render_function_body = build_render_function_body(grammar);

    quote!({
        let mut render_buffer = String::new();
        #[allow(non_snake_case)]
        let OUTPUT = &mut render_buffer;
        #render_function_body
        render_buffer
    })
    .into()
}
