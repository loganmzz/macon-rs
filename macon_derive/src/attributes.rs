use crate::common::{
    ResultErrorContext,
    Setting,
};
use std::collections::HashMap;
use proc_macro2::Span;
use syn::{
    meta::ParseNestedMeta,
    spanned::Spanned,
    Attribute,
    DeriveInput,
    Field,
    Ident,
    Meta,
    MetaList,
    Result,
    Type,
};

#[derive(Debug, Default, PartialEq)]
pub struct StructBuilder {
    mode: Setting<String>,
    default: Setting<()>,
    fields: StructBuilderFields,
}

#[derive(Debug, Default, PartialEq)]
pub struct StructBuilderFields {
    option: Setting<()>,
    into: Setting<()>,
}

#[derive(Debug, Default, PartialEq)]
pub struct FieldBuilder {
    option: Setting<Type>,
    default: Setting<()>,
    into: Setting<()>,
}

#[derive(Debug, Default)]
pub struct Derives {
    types: HashMap<String, Span>,
}

impl StructBuilder {
    pub fn mode(&self) -> &Setting<String> {
        &self.mode
    }
    pub fn mode_mut(&mut self) -> &mut Setting<String> {
        &mut self.mode
    }

    pub fn default(&self) -> &Setting<()> {
        &self.default
    }
    pub fn default_mut(&mut self) -> &Setting<()> {
        &mut self.default
    }

    pub fn fields(&self) -> &StructBuilderFields {
        &self.fields
    }
    pub fn fields_mut(&mut self) -> &mut StructBuilderFields {
        &mut self.fields
    }

    pub fn from_input(derive: &DeriveInput) -> Result<Self> {
        let mut struct_attributes: Self = Default::default();
        for attr in &derive.attrs {
            struct_attributes.with_attribute(attr)?;
        }
        Ok(struct_attributes)
    }
    fn with_attribute(&mut self, attribute: &Attribute) -> Result<()> {
        if let Meta::List(ref meta_list) = attribute.meta {
            if meta_list.path.is_ident("builder") {
                self.with_meta_list(meta_list)?;
            }
        }
        Ok(())
    }
    fn with_meta_list(&mut self, meta_list: &MetaList) -> Result<()> {
        meta_list.parse_nested_meta(|nested| {
            if nested.path.is_ident("mode") {
                let value: Ident = nested
                    .value()
                    .map_err_context("Unable to parse mode value for struct builder attribute")?
                    .parse()
                    .map_err_context("Unable to parse into Ident mode value for struct builder attribute")?;
                self.mode = Setting::enable(value.to_string(), value.span());
            } else if nested.path.is_ident("Option") {
                //TODO proc_macro_diagnostic https://github.com/rust-lang/rust/issues/54140
                eprintln!("WARNING: macon: Option at struct level be included in nested fields. e.g. `#[builder(fields(Option))]`");
                self.fields.option = Setting::<()>::from_parse_nested_meta(nested)
                    .map_err_context("Unable to parse Option for struct builder attribute")?;
            } else if nested.path.is_ident("Default") {
                self.default = Setting::<()>::from_parse_nested_meta(nested)
                    .map_err_context("Unable to parse Default for struct builder attribute")?;
            } else if nested.path.is_ident("Into") {
                //TODO proc_macro_diagnostic https://github.com/rust-lang/rust/issues/54140
                eprintln!("WARNING: macon: Into at struct level be included in nested fields. e.g. `#[builder(fields(Option))]`");
                self.fields.into = Setting::<()>::from_parse_nested_meta(nested)
                    .map_err_context("Unable to parse Into for struct builder attribute")?;
            } else if nested.path.is_ident("fields") {
                self.fields.with_parse_nested_meta(nested)?;
            } else {
                return Err(nested.error(format!("Unsupported struct builder attribute option: {:?}", nested.path)));
            }
            Ok(())
        })
    }
}

impl StructBuilderFields {
    pub fn option(&self) -> &Setting<()> {
        &self.option
    }
    pub fn option_mut(&mut self) -> &mut Setting<()> {
        &mut self.option
    }

    pub fn into_(&self) -> &Setting<()> {
        &self.into
    }
    pub fn into_mut(&mut self) -> &mut Setting<()> {
        &mut self.into
    }

    fn with_parse_nested_meta(&mut self, attr: ParseNestedMeta) -> Result<()> {
        attr.parse_nested_meta(|nested| {
            if nested.path.is_ident("Option") {
                self.option = Setting::<()>::from_parse_nested_meta(nested)
                    .map_err_context("Unable to parse Option for fields struct builder attribute")?;
            } else if nested.path.is_ident("Into") {
                self.into = Setting::<()>::from_parse_nested_meta(nested)
                    .map_err_context("Unable to parse Into for fields struct builder attribute")?;
            } else {
                return Err(nested.error(format!("Unsupported fields struct builder attribute option: {:?}", nested.path)));
            }
            Ok(())
        })?;
        Ok(())
    }
}

impl FieldBuilder {
    pub fn option(&self) -> &Setting<Type> {
        &self.option
    }

    pub fn default(&self) -> &Setting<()> {
        &self.default
    }

    pub fn into_(&self) -> &Setting<()> {
        &self.into
    }

    pub fn from_field(field: &Field) -> Result<Self> {
        let mut field_attributes: Self = Default::default();
        for attr in &field.attrs {
            field_attributes.with_attribute(attr)?;
        }
        Ok(field_attributes)
    }
    fn with_attribute(&mut self, attr: &Attribute) -> Result<()> {
        if let Meta::List(ref meta_list) = attr.meta {
            if meta_list.path.is_ident("builder") {
                self.with_meta_list(meta_list)?;
            }
        }
        Ok(())
    }
    fn with_meta_list(&mut self, meta_list: &MetaList) -> Result<()> {
        meta_list.parse_nested_meta(|nested| {
            if nested.path.is_ident("Option") {
                if self.option.is_defined() {
                    return Err(nested.error(format!("Option has been already specified ({:?}) for field builder attribute", self.option)));
                }
                self.option = Setting::<Type>::from_parse_nested_meta(nested)
                    .map_err_context("Unable to parse Option for field builder attribute")?
                    .and_then(|ty, span| {
                        match ty {
                            Type::Tuple(ref typetuple) => {
                                if typetuple.elems.is_empty() {
                                    Setting::enable(ty, span)
                                } else {
                                    Setting::disable(span)
                                }
                            },
                            _ => Setting::enable(ty, span),
                        }
                    });
            } else if nested.path.is_ident("Default") {
                if self.default.is_defined() {
                    return Err(nested.error(format!("Default has been already specified ({:?}) for field builder attribute", self.option)));
                }
                self.default = Setting::<()>::from_parse_nested_meta(nested)
                    .map_err_context(format!("Unable to parse Default value for field builder attribute"))?;
            } else if nested.path.is_ident("Into") {
                if self.into.is_defined() {
                    return Err(nested.error(format!("Into has been already specified ({:?}) for field builder attribute", self.option)));
                }
                self.into = Setting::<()>::from_parse_nested_meta(nested)
                    .map_err_context(format!("Unable to parse Into value for field builder attribute"))?;
            } else {
                return Err(nested.error(format!("Unsupported option {:?} for field builder attribute", nested.path)));
            }
            Ok(())
        })
    }
}

impl Derives {
    pub fn types(&self) -> &HashMap<String, Span> {
        &self.types
    }
    pub fn types_mut(&mut self) -> &mut HashMap<String, Span> {
        &mut self.types
    }

    pub fn get_type(&self, ty: &str) -> Option<&Span> {
        self.types.get(ty)
    }

    pub fn from_input(input: &DeriveInput) -> Result<Self> {
        let mut derive: Self = Default::default();
        for attr in &input.attrs {
            derive.with_attribute(attr)?;
        }
        Ok(derive)
    }
    fn with_attribute(&mut self, attribute: &Attribute) -> Result<()> {
        if let Meta::List(ref meta_list) = attribute.meta {
            if meta_list.path.is_ident("derive") {
                self.with_meta_list(meta_list)?;
            }
        }
        Ok(())
    }
    fn with_meta_list(&mut self, meta_list: &MetaList) -> Result<()> {
        meta_list.parse_nested_meta(|nested| {
            let span = nested.path.span();
            let ty: String = nested
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
            self.types.insert(ty, span);
            Ok(())
        })
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashSet;

    use super::*;
    use quote::quote;
    use syn::{
        parse::Parser,
        parse_quote,
        parse_str,
        DeriveInput,
    };

    fn span() -> Span {
        Span::call_site()
    }

    #[test]
    fn struct_builder_attribute() {
        let derive_input: DeriveInput = parse_quote! {
            struct Foobar;
        };
        let builder = StructBuilder::from_input(&derive_input)
            .expect("StructBuilder::from_input");
        assert_eq!(
            builder.mode,
            Setting::undefined(),
            "mode",
        );
        assert_eq!(
            builder.default,
            Setting::undefined(),
            "default",
        );
        assert_eq!(
            builder.fields.into,
            Setting::undefined(),
            "fields.into",
        );
        assert_eq!(
            builder.fields.option,
            Setting::undefined(),
            "fields.option",
        );
    }

    #[test]
    fn struct_builder_attribute_mode() {
        let derive_input: DeriveInput = parse_quote! {
            #[builder(mode=Foo)]
            struct Foobar;
        };
        let builder = StructBuilder::from_input(&derive_input)
            .expect("StructBuilder::from_input");
        assert_eq!(
            builder.mode,
            Setting::enable("Foo".to_string(), span()),
            "mode",
        );
        assert_eq!(
            builder.default,
            Setting::undefined(),
            "default",
        );
        assert_eq!(
            builder.fields.into,
            Setting::undefined(),
            "fields.into",
        );
        assert_eq!(
            builder.fields.option,
            Setting::undefined(),
            "fields.option",
        );
    }

    #[test]
    fn struct_builder_attribute_default_enabled() {
        let derive_input: DeriveInput = parse_quote! {
            #[builder(Default)]
            struct Foobar;
        };
        let builder = StructBuilder::from_input(&derive_input)
            .expect("StructBuilder::from_input");
        assert_eq!(
            builder.mode,
            Setting::undefined(),
            "mode",
        );
        assert_eq!(
            builder.default,
            Setting::enable((), span()),
            "default",
        );
        assert_eq!(
            builder.fields.into,
            Setting::undefined(),
            "fields.into",
        );
        assert_eq!(
            builder.fields.option,
            Setting::undefined(),
            "fields.option",
        );
    }

    #[test]
    fn struct_builder_attribute_default_disabled() {
        let derive_input: DeriveInput = parse_quote! {
            #[builder(Default=!)]
            struct Foobar;
        };
        let builder = StructBuilder::from_input(&derive_input)
            .expect("StructBuilder::from_input");
        assert_eq!(
            builder.mode,
            Setting::undefined(),
            "mode",
        );
        assert_eq!(
            builder.default,
            Setting::disable(span()),
            "default",
        );

        assert_eq!(
            builder.fields.into,
            Setting::undefined(),
            "fields.into",
        );
        assert_eq!(
            builder.fields.option,
            Setting::undefined(),
            "fields.option",
        );
    }

    #[test]
    fn struct_builder_attribute_into_enabled() {
        let derive_input: DeriveInput = parse_quote! {
            #[builder(Into)]
            struct Foobar;
        };
        let builder = StructBuilder::from_input(&derive_input)
            .expect("StructBuilder::from_input");
        assert_eq!(
            builder.mode,
            Setting::undefined(),
            "mode",
        );
        assert_eq!(
            builder.default,
            Setting::undefined(),
            "default",
        );
        assert_eq!(
            builder.fields.into,
            Setting::enable((), span()),
            "fields.into",
        );
        assert_eq!(
            builder.fields.option,
            Setting::undefined(),
            "fields.option",
        );
    }

    #[test]
    fn struct_builder_attribute_into_disabled() {
        let derive_input: DeriveInput = parse_quote! {
            #[builder(Into=!)]
            struct Foobar;
        };
        let builder = StructBuilder::from_input(&derive_input)
            .expect("StructBuilder::from_input");
        assert_eq!(
            builder.mode,
            Setting::undefined(),
            "mode",
        );
        assert_eq!(
            builder.default,
            Setting::undefined(),
            "default",
        );
        assert_eq!(
            builder.fields.into,
            Setting::disable(span()),
            "fields.into",
        );
        assert_eq!(
            builder.fields.option,
            Setting::undefined(),
            "fields.option",
        );
    }

    #[test]
    fn struct_builder_attribute_option_enabled() {
        let derive_input: DeriveInput = parse_quote! {
            #[builder(Option)]
            struct Foobar;
        };
        let builder = StructBuilder::from_input(&derive_input)
            .expect("StructBuilder::from_input");
        assert_eq!(
            builder.mode,
            Setting::undefined(),
            "mode",
        );
        assert_eq!(
            builder.default,
            Setting::undefined(),
            "default",
        );
        assert_eq!(
            builder.fields.into,
            Setting::undefined(),
            "fields.into",
        );
        assert_eq!(
            builder.fields.option,
            Setting::enable((), span()),
            "fields.option",
        );
    }

    #[test]
    fn struct_builder_attribute_option_disabled() {
        let derive_input: DeriveInput = parse_quote! {
            #[builder(Option=!)]
            struct Foobar;
        };
        let builder = StructBuilder::from_input(&derive_input)
            .expect("StructBuilder::from_input");
        assert_eq!(
            builder.mode,
            Setting::undefined(),
            "mode",
        );
        assert_eq!(
            builder.default,
            Setting::undefined(),
            "default",
        );
        assert_eq!(
            builder.fields.into,
            Setting::undefined(),
            "fields.into",
        );
        assert_eq!(
            builder.fields.option,
            Setting::disable(span()),
            "fields.option",
        );
    }

    #[test]
    fn struct_builder_attribute_fields_into_enabled() {
        let derive_input: DeriveInput = parse_quote! {
            #[builder(fields(Into))]
            struct Foobar;
        };
        let builder = StructBuilder::from_input(&derive_input)
            .expect("StructBuilder::from_input");
        assert_eq!(
            builder.mode,
            Setting::undefined(),
            "mode",
        );
        assert_eq!(
            builder.default,
            Setting::undefined(),
            "default",
        );
        assert_eq!(
            builder.fields.into,
            Setting::enable((), span()),
            "fields.into",
        );
        assert_eq!(
            builder.fields.option,
            Setting::undefined(),
            "fields.option",
        );
    }

    #[test]
    fn struct_builder_attribute_fields_into_disabled() {
        let derive_input: DeriveInput = parse_quote! {
            #[builder(fields(Into=!))]
            struct Foobar;
        };
        let builder = StructBuilder::from_input(&derive_input)
            .expect("StructBuilder::from_input");
        assert_eq!(
            builder.mode,
            Setting::undefined(),
            "mode",
        );
        assert_eq!(
            builder.default,
            Setting::undefined(),
            "default",
        );
        assert_eq!(
            builder.fields.into,
            Setting::disable(span()),
            "fields.into",
        );
        assert_eq!(
            builder.fields.option,
            Setting::undefined(),
            "fields.option",
        );
    }

    #[test]
    fn struct_builder_attribute_fields_option_enabled() {
        let derive_input: DeriveInput = parse_quote! {
            #[builder(fields(Option))]
            struct Foobar;
        };
        let builder = StructBuilder::from_input(&derive_input)
            .expect("StructBuilder::from_input");
        assert_eq!(
            builder.mode,
            Setting::undefined(),
            "mode",
        );
        assert_eq!(
            builder.default,
            Setting::undefined(),
            "default",
        );
        assert_eq!(
            builder.fields.into,
            Setting::undefined(),
            "fields.into",
        );
        assert_eq!(
            builder.fields.option,
            Setting::enable((), span()),
            "fields.option",
        );
    }

    #[test]
    fn struct_builder_attribute_fields_option_disabled() {
        let derive_input: DeriveInput = parse_quote! {
            #[builder(fields(Option=!))]
            struct Foobar;
        };
        let builder = StructBuilder::from_input(&derive_input)
            .expect("StructBuilder::from_input");
        assert_eq!(
            builder.mode,
            Setting::undefined(),
            "mode",
        );
        assert_eq!(
            builder.default,
            Setting::undefined(),
            "default",
        );
        assert_eq!(
            builder.fields.into,
            Setting::undefined(),
            "fields.into",
        );
        assert_eq!(
            builder.fields.option,
            Setting::disable(span()),
            "fields.option",
        );
    }

    #[test]
    fn field_builder_attribute() {
        let field = Field::parse_named.parse2(quote! {
            foobar: Foobar
        })
            .expect("Field::parse_unnamed");
        let builder = FieldBuilder::from_field(&field)
            .expect("FieldBuilder::from_field");
        assert_eq!(
            builder.default,
            Setting::undefined(),
            "default",
        );
        assert_eq!(
            builder.into,
            Setting::undefined(),
            "into",
        );
        assert_eq!(
            builder.option,
            Setting::undefined(),
            "option",
        );
    }

    #[test]
    fn field_builder_attribute_default_enabled() {
        let field = Field::parse_named.parse2(quote! {
            #[builder(Default)]
            foobar: Foobar
        })
            .expect("Field::parse_unnamed");
        let builder = FieldBuilder::from_field(&field)
            .expect("FieldBuilder::from_field");
        assert_eq!(
            builder.default,
            Setting::enable((), span()),
            "default",
        );
        assert_eq!(
            builder.into,
            Setting::undefined(),
            "into",
        );
        assert_eq!(
            builder.option,
            Setting::undefined(),
            "option",
        );
    }

    #[test]
    fn field_builder_attribute_default_disabled() {
        let field = Field::parse_named.parse2(quote! {
            #[builder(Default=!)]
            foobar: Foobar
        })
            .expect("Field::parse_unnamed");
        let builder = FieldBuilder::from_field(&field)
            .expect("FieldBuilder::from_field");
        assert_eq!(
            builder.default,
            Setting::disable(span()),
            "default",
        );
        assert_eq!(
            builder.into,
            Setting::undefined(),
            "into",
        );
        assert_eq!(
            builder.option,
            Setting::undefined(),
            "option",
        );
    }

    #[test]
    fn field_builder_attribute_into_enabled() {
        let field = Field::parse_named.parse2(quote! {
            #[builder(Into)]
            foobar: Foobar
        })
            .expect("Field::parse_unnamed");
        let builder = FieldBuilder::from_field(&field)
            .expect("FieldBuilder::from_field");
        assert_eq!(
            builder.default,
            Setting::undefined(),
            "default",
        );
        assert_eq!(
            builder.into,
            Setting::enable((), span()),
            "into",
        );
        assert_eq!(
            builder.option,
            Setting::undefined(),
            "option",
        );
    }

    #[test]
    fn field_builder_attribute_into_disabled() {
        let field = Field::parse_named.parse2(quote! {
            #[builder(Into=!)]
            foobar: Foobar
        })
            .expect("Field::parse_unnamed");
        let builder = FieldBuilder::from_field(&field)
            .expect("FieldBuilder::from_field");
        assert_eq!(
            builder.default,
            Setting::undefined(),
            "default",
        );
        assert_eq!(
            builder.into,
            Setting::disable(span()),
            "into",
        );
        assert_eq!(
            builder.option,
            Setting::undefined(),
            "option",
        );
    }

    #[test]
    fn field_builder_attribute_option_enabled() {
        let field = Field::parse_named.parse2(quote! {
            #[builder(Option=Bar)]
            foobar: Foobar
        })
            .expect("Field::parse_unnamed");
        let builder = FieldBuilder::from_field(&field)
            .expect("FieldBuilder::from_field");
        assert_eq!(
            builder.default,
            Setting::undefined(),
            "default",
        );
        assert_eq!(
            builder.into,
            Setting::undefined(),
            "into",
        );
        assert_eq!(
            builder.option,
            Setting::enable(parse_str::<Type>("Bar").unwrap(), span()),
            "option",
        );
    }

    #[test]
    fn field_builder_attribute_option_disabled() {
        let field = Field::parse_named.parse2(quote! {
            #[builder(Option=!)]
            foobar: Foobar
        })
            .expect("Field::parse_unnamed");
        let builder = FieldBuilder::from_field(&field)
            .expect("FieldBuilder::from_field");
        assert_eq!(
            builder.default,
            Setting::undefined(),
            "default",
        );
        assert_eq!(
            builder.into,
            Setting::undefined(),
            "into",
        );
        assert_eq!(
            builder.option,
            Setting::disable(span()),
            "option",
        );
    }

    #[test]
    fn derives() {
        let derive_input: DeriveInput = parse_quote! {
            #[derive(One,Two,Three)]
            struct Foobar;
        };
        let derives = Derives::from_input(&derive_input)
            .expect("Derives::from_input");
        assert_eq!(
            derives.types.keys().map(|s| s.as_str()).collect::<HashSet<_>>(),
            HashSet::from(["One", "Two", "Three"]),
            "types",
        )
    }
}
