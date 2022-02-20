#![feature(proc_macro_span)]
#![feature(extend_one)]
#![feature(proc_macro_span_shrink)]

extern crate proc_macro;
use std::iter::Peekable;
use proc_macro::{TokenStream, Delimiter, TokenTree::Punct, TokenTree::Group};
use proc_macro2::TokenStream as P2TokenStream;
use proc_macro2::{Group as P2Group, Delimiter as P2Delimiter};
use quote::quote;

mod traits;
use crate::traits::GetSource;

struct Parser {
    token_iter : Peekable<<proc_macro::TokenStream as IntoIterator>::IntoIter>,
    span_ptr : proc_macro::Span,
    in_element : bool
}

impl Parser {
    fn new(token_stream : proc_macro::TokenStream) -> Self {
        let mut token_iter = token_stream.into_iter().peekable();
        let span_ptr = token_iter.peek().unwrap().span();
        Self {
            token_iter,
            span_ptr,
            in_element : false
        }
    }
    fn move_span_ptr(&mut self) {
        if let Some(next_tokentree) = self.token_iter.peek() {
            self.span_ptr = next_tokentree.span();
        };
    }
}

impl Iterator for Parser {
    type Item = P2TokenStream;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.token_iter.next()? {
                Group(group) if group.delimiter() == Delimiter::Brace => {
                    let mut new_tokenstream = if let Some(html) = group.source_text_until(self.span_ptr) {
                        quote!{
                            html.push_str(#html);
                        }
                    } else {
                        P2TokenStream::new()
                    };

                    let code = P2Group::new(P2Delimiter::Brace, P2TokenStream::from(group.stream()));

                    new_tokenstream.extend_one(if self.in_element {
                            self.in_element = false;
                            quote!{
                                html.push('"');
                                html.push_str(&#code.to_string());
                                html.push('"');
                            }
                        } else {
                            quote!{
                                html.push_str(&#code.to_string());
                            }
                        }
                    );

                    self.move_span_ptr();

                    return Some(new_tokenstream)
                },
                Punct(punct) if punct.as_char() == '=' => {
                    self.in_element = true
                },
                tokentree => {
                    self.in_element = false;
                    if self.token_iter.peek().is_none() {
                        let html = tokentree.span().join(self.span_ptr)?.source_text()?;
                        return Some(
                            quote!{
                                html.push_str(#html);
                            }
                        )
                    }
                }
            }
        }
    }
}

#[proc_macro]
pub fn html(input : TokenStream) -> TokenStream {
    let parser = Parser::new(input);
    quote! {
        {
            let mut html = String::new();
            #(#parser)*
            html
        }
    }.into()
}