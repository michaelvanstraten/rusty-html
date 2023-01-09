macro_rules! err_to_token_stream {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(err) => {
                return syn::Error::new(proc_macro2::Span::call_site(), err.to_string())
                    .to_compile_error()
                    .into()
            }
        }
    };
    ($result:expr, $blame_on:expr) => {
        match $result {
            Ok(value) => value,
            Err(err) => {
                return syn::Error::new($blame_on, err.to_string())
                    .to_compile_error()
                    .into()
            }
        }
    };
}

macro_rules! syn_err_to_token_stream {
    ($result:expr) => {
        match $result {
            Ok(value) => value,
            Err(err) => return err.to_compile_error().into(),
        }
    };
}

macro_rules! push_string_to_token_stream {
    ($string:ident, $token_stream:ident) => {
        if !$string.is_empty() {
            $token_stream.append_all(quote!(
                ::rusty_html::render::RenderBuffer::push_str(
                    OUTPUT,
                    #$string
                );
            ));
            $string.clear();
        }
    }
}

pub(crate) use err_to_token_stream;
pub(crate) use push_string_to_token_stream;
pub(crate) use syn_err_to_token_stream;
