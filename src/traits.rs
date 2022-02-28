use proc_macro::{Group, TokenTree};

pub trait GetSource {
    fn span(&self) -> proc_macro::Span;
    fn get_source(&self) -> String {
        self.span().source_text().unwrap()
    }
    fn source_text_until(&self, back : proc_macro::Span) -> Option<String> {
        if !self.span().eq(&back) {
            self.span().before().join(back).unwrap().source_text()
        } else {
            None
        }
    }
}

impl GetSource for TokenTree {
    fn span(&self) -> proc_macro::Span {
        self.span()
    }
}

impl GetSource for Group {
    fn span(&self) -> proc_macro::Span {
        self.span()
    }
}
