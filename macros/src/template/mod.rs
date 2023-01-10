mod arguments;

use std::fs::File;
use std::io::Error as IOError;
use std::io::Read;

use pest::Parser;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::token::Comma;
use syn::AttributeArgs;
use syn::{DeriveInput, GenericParam};

use crate::grammar::Grammar;
use crate::grammar::Rule;
use crate::helper_macros::err_to_token_stream;
use crate::helper_macros::syn_err_to_token_stream;
use crate::build::build_render_function_body;
use crate::template::arguments::TemplateArgs;

pub(crate) fn build_template(input: DeriveInput, mut attributes: AttributeArgs) -> TokenStream {
    let arguments = syn_err_to_token_stream!(TemplateArgs::try_from(&mut attributes));
    let template_data = err_to_token_stream!(
        read_template_data(&arguments),
        arguments._template_path_span
    );

    let grammar = err_to_token_stream!(
        Grammar::parse(Rule::html, &template_data),
        arguments._template_path_span
    );

    let body_of_render_function = build_render_function_body(grammar);

    let mut generics = input.generics.clone();
    if !generics.params.empty_or_trailing() {
        generics.params.push_punct(Comma(input.ident.span()));
    }
    let generic_param = quote!(B: rusty_html::render::RenderBuffer).into();
    generics
        .params
        .push_value(parse_macro_input!(generic_param as GenericParam));

    let ident = &input.ident;
    let (_, type_generics, where_clauses) = input.generics.split_for_impl();
    let (impl_generics, _, _) = generics.split_for_impl();

    quote! {
        #input

        impl #impl_generics rusty_html::render::Render<B> for #ident #type_generics #where_clauses {
            fn render_to_buf(&self, buf: &mut B) {
                #[allow(non_snake_case)]
                let mut OUTPUT = buf;
                #body_of_render_function
            }
        }
    }
    .into()
}

fn read_template_data(arguments: &TemplateArgs) -> Result<String, IOError> {
    let mut template_data = String::new();
    File::open(&arguments.template_path)?.read_to_string(&mut template_data)?;
    Ok(template_data)
}
