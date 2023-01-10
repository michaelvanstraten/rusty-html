#![cfg_attr(feature = "inline-html", feature(proc_macro_span))]
// #![cfg_attr(feature = "better-errors", feature(proc_macro_diagnostic))]

mod inline_html;

mod build;
mod grammar;
mod helper_macros;
mod template;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use syn::AttributeArgs;
use syn::DeriveInput;

#[proc_macro_attribute]
pub fn template(attribute_stream: TokenStream, input_stream: TokenStream) -> TokenStream {
    template::build_template(
        parse_macro_input!(input_stream as DeriveInput),
        parse_macro_input!(attribute_stream as AttributeArgs),
    )
}

#[proc_macro]
pub fn html(input_stream: TokenStream) -> TokenStream {
    inline_html::build_render_scope(input_stream)
}

