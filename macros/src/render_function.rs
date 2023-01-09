use std::str::FromStr;

use pest::iterators::Pairs;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::TokenStreamExt;
use syn::Error as SynError;

use crate::grammar::Rule;
use crate::helper_macros::err_to_token_stream;
use crate::helper_macros::push_string_to_token_stream;

pub(crate) fn build_render_function_body(grammer_pairs: Pairs<Rule>) -> TokenStream {
    let mut html_buffer = String::new();

    let mut render_tree = build_render_tree(grammer_pairs, &mut html_buffer);

    push_string_to_token_stream!(html_buffer, render_tree);

    render_tree
}

fn build_render_tree(grammer_pairs: Pairs<Rule>, html_buffer: &mut String) -> TokenStream {
    let mut render_tree = TokenStream::new();

    for pair in grammer_pairs {
        match pair.as_rule() {
            Rule::node_element | Rule::el_normal_start => {
                render_tree.append_all(build_render_tree(pair.into_inner(), html_buffer));
            }
            Rule::rust_scope => {
                push_string_to_token_stream!(html_buffer, render_tree);
                let rust_scope = build_rust_scope(pair.into_inner(), html_buffer);
                render_tree.append_all(quote!(
                    ::rusty_html::render::Render::render_to_buf_escaped(
                        &#rust_scope,
                        OUTPUT
                    );
                ));
            }
            Rule::attr => {
                html_buffer.push_str(" ");
                render_tree.append_all(build_render_tree(pair.into_inner(), html_buffer));
            }
            Rule::code_attr => {
                html_buffer.push_str("\"");
                render_tree.append_all(build_render_tree(pair.into_inner(), html_buffer));
                html_buffer.push_str("\"");
            }
            _ => html_buffer.push_str(pair.as_str()),
        }
    }
    render_tree
}

fn build_rust_scope(grammer_pairs: Pairs<Rule>, html_buffer: &mut String) -> TokenStream {
    let mut rust_scope = TokenStream::new();
    for pair in grammer_pairs {
        match pair.as_rule() {
            Rule::rust_code => {
                let code_body = err_to_token_stream!(TokenStream::from_str(pair.as_str()));
                rust_scope.append_all(quote!(#code_body))
            }
            Rule::rust_scope => {
                rust_scope.append_all(build_rust_scope(pair.into_inner(), html_buffer));
            }
            Rule::node_element => {
                rust_scope.append_all(build_render_tree(pair.into_inner(), html_buffer));
            }
            _ => {
                return SynError::new(Span::call_site(), "Unexpected grammer pair")
                    .to_compile_error()
                    .into()
            }
        }
    }
    push_string_to_token_stream!(html_buffer, rust_scope);
    quote!({#rust_scope})
}
