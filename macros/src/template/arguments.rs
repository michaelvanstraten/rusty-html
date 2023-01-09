use proc_macro2::Span;
use syn::{Error as SynError, Lit, NestedMeta, Meta};

#[derive(Debug)]
pub(crate) struct TemplateArgs {
    pub template_path: String,
    pub _template_path_span: Span,
}

impl TryFrom<&mut Vec<NestedMeta>> for TemplateArgs {
    type Error = SynError;

    fn try_from(value: &mut Vec<NestedMeta>) -> Result<Self, Self::Error> {
        let mut value_iter = value.into_iter();

        let mut template_path = None;
        let mut template_path_span = None;
        let mut parse_template_path = |literal: &mut Lit| {
            if let Lit::Str(template_path_value) = literal {
                template_path_span = Some(template_path_value.span());
                Ok(Some(template_path_value.value()))
            } else {
                Err(SynError::new_spanned(
                    literal,
                    "Expected a literal string path to a template file.",
                ))
            }
        };


        while let Some(value) = value_iter.next() {
            match value {
                NestedMeta::Meta(meta) => {
                    if let Meta::NameValue(named_value) = meta {

                        if named_value.path.is_ident("template_path") {
                            template_path = parse_template_path(&mut named_value.lit)?;
                        } else {
                            return Err(SynError::new_spanned(
                                &named_value.path,
                                "Unexpected literal, the Arguments do not include such an Option",
                            ));
                        }
                    } else {
                        return Err(SynError::new_spanned(value, "Unexpected token"));
                    }
                },
                NestedMeta::Lit(literal) => {
                    if template_path == None {
                        template_path = parse_template_path(literal)?;
                    } else {
                        return Err(SynError::new_spanned(
                            value,
                            "Unexpected literal, all arguments were already provided",
                        ));
                    }
                },
            }
        }

        Ok(Self {
            template_path: match template_path {
                Some(template_path) => template_path,
                None => return Err(SynError::new(
                    Span::call_site(),
                    "template_path not set",
                )),
            },
            _template_path_span: template_path_span.unwrap(),
        })
    }
}
