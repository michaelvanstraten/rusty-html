use pest::Parser;
use proc_macro::TokenStream;
use quote::quote;

use crate::grammar::Grammar;
use crate::grammar::Rule;
use crate::helper_macros::err_to_token_stream;
use crate::render_function::build_render_function_body;

pub(crate) fn build_render_scope(input_stream: TokenStream) -> TokenStream {
    let mut input_stream_inter = input_stream.into_iter();

    if let (Some(first_token), Some(last_token)) =
        (input_stream_inter.next(), input_stream_inter.last())
    {
        if let Some(combind_span) = first_token.span().join(last_token.span()) {
            if let Some(template_data) = combind_span.source_text() {
                let grammar = err_to_token_stream!(
                    Grammar::parse(Rule::html, &template_data),
                    combind_span.into()
                );
                let render_function_body = build_render_function_body(grammar);
                return quote!({
                    let mut render_buffer = String::new();
                    let OUTPUT = &mut render_buffer;
                    #render_function_body
                    render_buffer
                })
                .into();
            }
        }
    }

    quote!("").into()
}
