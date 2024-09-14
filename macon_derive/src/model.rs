use crate::common::{
    ResultErrorContext,
    Setting,
};
use crate::attributes::{
    Derives,
    FieldBuilder,
    StructBuilder,
};
use std::borrow::Cow;
use proc_macro2::{
    Delimiter,
    Group,
    Literal,
    TokenStream,
};
use quote::{
    format_ident,
    quote, ToTokens,
};
use syn::spanned::Spanned;
use syn::{
    Data,
    DeriveInput,
    Error,
    Field,
    Fields,
    GenericArgument,
    Ident,
    PathArguments,
    Result,
    Type,
    Visibility,
};

#[derive(Debug)]
pub struct Builder {
    /// Builder ident
    pub ident: Ident,
    /// Source struct ident
    pub target: Ident,
    /// Builder visibility
    pub vis: Visibility,
    /// Builder mode
    pub mode: Mode,
    /// Builder fields
    pub properties: Properties,
    /// Is Tuple struct `(a, b, c)` or Named one `{ a:A, b:B, c:C }`
    pub is_tuple: bool,
    /// Is Default implemented for struct
    pub default: Setting<()>,
}

#[derive(Debug,PartialEq)]
pub enum Mode {
    Typestate,
    Result,
    Panic,
}

#[derive(Clone,Copy,Debug,PartialEq,)]
pub enum Setter {
    Standard,
    None,
    Keep,
    Default,
}

#[derive(Debug,Default)]
pub struct PropertySettings {
    pub option: Setting<Type>,
    pub default: Setting<()>,
    pub into: Setting<()>,
}

#[derive(Debug,Default,)]
pub struct Properties {
    /// Is Tuple struct `(a,b,c)` or Named one `{ a:A, b:B, c:C }`
    pub is_tuple: bool,
    /// Is Default supported for struct
    pub default: Setting<()>,
    /// Is Into supported for fields
    pub into: Setting<()>,
    /// Is Option supported for fields
    pub option: Setting<()>,
    /// Struct fields
    items: Vec<Property>,
}
impl ::core::ops::Deref for Properties {
    type Target = Vec<Property>;
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
impl ::core::ops::DerefMut for Properties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}
impl Properties {
    pub fn to_token<F: Fn(&Property)->TokenStream>(&self, to_token: F) -> TokenStream {
        self
            .iter()
            .map(to_token)
            .collect()
    }
}


#[derive(Debug)]
pub struct Property {
    /// Field order (mostly for tuple)
    pub ordinal: usize,
    /// Variable name when refering to this field
    pub name: String,
    /// Variable name when refering to this field
    pub ident: Ident,
    /// Field type (may be a wrapper)
    pub ty: Type,
    /// Is Tuple struct field `(a,b,c)` or Named one `{ a:A, b:B, c:C }`
    pub is_tuple: bool,
    /// Is Option and associated wrapped type
    pub option: Setting<Type>,
    /// Is Default supported for field
    pub default: Setting<()>,
    /// Is Into supported for field
    pub into: Setting<()>,
    /// Is Default supported for struct
    pub struct_default: Setting<()>,
}

impl TryFrom<&Setting<String>> for Mode {
    type Error = Error;
    fn try_from(value: &Setting<String>) -> Result<Self> {
        Ok(match value {
            Setting::Undefined => Mode::default(),
            Setting::Enabled { value, span } => {
                match value.as_str() {
                    "Typestate" => Mode::Typestate,
                    "Result" => Mode::Result,
                    "Panic" => Mode::Panic,
                    _ => return Err(Error::new(span.clone(), format!("Unsupported mode value {} for struct builder attribute", value))),
                }
            }
            Setting::Disabled { span } => return Err(Error::new(span.clone(), format!("Unsupported disabled mode for struct builder attribute"))),
        })
    }
}
impl Default for Mode {
    fn default() -> Self {
        Self::Typestate
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            ident: format_ident!("AnonymousBuilder"),
            target: format_ident!("Anonymous"),
            vis: Visibility::Inherited,
            mode: Default::default(),
            properties: Default::default(),
            is_tuple: false,
            default: Default::default(),
        }
    }
}

impl Builder {
    pub fn from_input(derive: DeriveInput) -> Result<Self> {
        let mut this = Self::default();
        this.target = derive.ident.clone();
        this.ident = format_ident!("{}Builder", this.target);
        this.vis = derive.vis.clone();
        let builder_attr = StructBuilder::from_input(&derive)?;
        let derives = Derives::from_input(&derive)?;
        this.with_attributes(builder_attr, derives)?;
        this.with_data(derive.data)?;
        Ok(this)
    }

    pub fn with_attributes(&mut self, builder: StructBuilder, derives: Derives) -> Result<()> {
        self.mode = builder.mode().try_into()?;
        self.set_default(builder.default().clone());

        self.properties.option = builder.fields().option().clone();
        self.properties.into   = builder.fields().into_().clone();

        if ! self.default.is_defined() {
            if let Some(span) = derives.get_type("Default") {
                self.set_default(Setting::enable((), span.clone()));
            }
        }
        Ok(())
    }

    pub fn with_data(&mut self, data: Data) -> Result<()> {
        match data {
            Data::Struct(data_struct) => {
                match data_struct.fields {
                    Fields::Named(fields_named) => {
                        self.set_is_tuple(false);
                        for (ordinal, field) in fields_named.named.into_iter().enumerate() {
                            let value = Property::from_field(self, false, ordinal, field)?;
                            self.properties.push(value);
                        }
                        Ok(())
                    },
                    Fields::Unit => {
                        Ok(())
                    },
                    Fields::Unnamed(fields_unamed) => {
                        self.set_is_tuple(true);
                        for (ordinal, field)  in fields_unamed.unnamed.into_iter().enumerate() {
                            let value = Property::from_field(self, true, ordinal, field)?;
                            self.properties.push(value);
                        }
                        Ok(())
                    },
                }
            },
            Data::Enum(e) => {
                Err(Error::new_spanned(e.enum_token, "enum is not supported"))
            },
            Data::Union(u) => {
                Err(Error::new_spanned(u.union_token, "union is not supported"))
            },
        }
    }

    pub fn set_is_tuple(&mut self, is_tuple: bool) {
        self.is_tuple = is_tuple;
        self.properties.is_tuple = is_tuple;
    }

    pub fn set_default(&mut self, default: Setting<()>) {
        self.default = default;
        self.properties.default = default;
    }
}

impl Property {
    pub fn from_field(builder: &Builder, is_tuple: bool, ordinal: usize, field: Field) -> Result<Self> {
        let ident = field.ident.clone().unwrap_or_else(|| format_ident!("v{}", ordinal));
        let name = ident.to_string();
        let span = field.ty.span();
        let builder_attribute = FieldBuilder::from_field(&field)
            .map_err_context(format!("Field {}", name))?;
        let option = if builder_attribute.option().is_undefined() {
            if builder.properties.option.is_disabled() {
                Setting::disable(span)
            } else if let Some(ty) = Self::get_option_arg(&field.ty) {
                Setting::enable(ty.clone(), span)
            } else {
                Setting::disable(span)
            }
        } else {
            builder_attribute.option().clone()
        };
        let default = if builder_attribute.default().is_undefined() {
            let default_types = crate::config::get().default_types();
            if default_types.match_type(&field.ty) {
                Setting::enable((), span)
            } else {
                Setting::disable(span)
            }
        } else {
            builder_attribute.default().clone()
        };
        let into = if builder_attribute.into_().is_undefined() {
            builder.properties.into.clone()
        } else {
            builder_attribute.into_().clone()
        };
        Ok(Self {
            ordinal,
            name,
            ident,
            ty: field.ty,
            is_tuple,
            option,
            default,
            into,
            struct_default: builder.default.clone(),
        })
    }

    pub fn get_option_arg(ty: &Type) -> Option<&Type> {
        if crate::config::get().option_types().match_type(ty) {
            match ty {
                Type::Path(typepath) => typepath
                    .path
                    .segments
                    .last()
                    .and_then(|path_segment| match path_segment.arguments {
                        PathArguments::AngleBracketed(ref args) => Some(args),
                        _ => None,
                    })
                    .and_then(|args| if args.args.len() == 1 {
                        if let Some(GenericArgument::Type(ty)) = args.args.first() {
                            Some(ty)
                        } else {
                            None
                        }
                    } else {
                        None
                    }),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn id(&self) -> TokenStream {
        if self.is_tuple {
            let literal = Literal::usize_unsuffixed(self.ordinal);
            quote!(#literal)
        } else {
            let ident = &self.ident;
            quote!(#ident)
        }
    }

    pub fn typevar(&self) -> Ident {
        format_ident!("{}", self.name.to_uppercase())
    }

    pub fn ty_into(&self) -> &Type {
        self.option.value().unwrap_or(&self.ty)
    }

    pub fn setter(&self) -> Cow<Ident> {
        if self.is_tuple {
            Cow::Owned(format_ident!("set{}", self.ordinal))
        } else {
            Cow::Borrowed(&self.ident)
        }
    }

    pub fn setter_none(&self) -> Ident {
        format_ident!("{}_none", self.setter())
    }

    pub fn setter_keep(&self) -> Ident {
        format_ident!("{}_keep", self.setter())
    }

    pub fn setter_default(&self) -> Ident {
        format_ident!("{}_default", self.setter())
    }

    pub fn prefix(&self) -> TokenStream {
        if self.is_tuple {
            quote!()
        } else {
            let ident = &self.ident;
            quote!(#ident:)
        }
    }

    pub fn is_required(&self) -> bool {
        ! self.option.is_enabled() &&
        ! self.default.is_enabled() &&
        ! self.struct_default.is_enabled() &&
        true
    }

    pub fn typestate(&self, is_var: Option<bool>) -> TokenStream {
        match is_var {
            None => quote!((),),
            Some(false) => {
                let ty = &self.ty;
                quote!(#ty,)
            },
            Some(true) => {
                let typevar = self.typevar();
                quote!(#typevar,)
            },
        }
    }

    pub fn typestate_struct_field(&self) -> TokenStream {
        let prefix = self.prefix();
        let ty = if ! self.is_required() {
            let mut ty = self.ty.to_token_stream();
            if self.default.is_enabled() {
                ty = quote!(::macon::Defaulting<#ty>);
            }
            if self.struct_default.is_enabled() {
                ty = quote!(::macon::Keeping<#ty>);
            }
            ty
        } else {
            self.typevar().to_token_stream()
        };
        quote!(#prefix #ty,)
    }

    pub fn typestate_optional_marker(&self) -> TokenStream {
        if ! self.is_required() {
            let typevar = self.typevar();
            quote!(#typevar,)
        } else {
            quote!()
        }
    }

    pub fn typestate_state(&self, target: &Property, ordered: bool, is_set: bool) -> TokenStream {
        if ordered {
            if is_set {
                if self.ordinal <= target.ordinal {
                    self.typestate(Some(false))
                } else {
                    self.typestate(None)
                }
            } else if self.ordinal < target.ordinal {
                    self.typestate(Some(false))
            } else {
                self.typestate(None)
            }
        } else if self.name == target.name {
            if is_set {
                self.typestate(Some(false))
            } else {
                self.typestate(None)
            }
        } else {
            self.typestate(Some(true))
        }
    }

    pub fn typestate_state_final(&self) -> TokenStream {
        self.typestate(Some(! self.is_required()))
    }

    pub fn typestate_setter_impl(&self, target: &Property) -> TokenStream {
        if self.name == target.name {
            quote!()
        } else {
            self.typestate(Some(true))
        }
    }

    pub fn typestate_assign(&self, target: &Property, setter: Setter) -> TokenStream {
        let prefix = self.prefix();
        let ident = &self.ident;
        let value = if self.name == target.name {
            match setter {
                Setter::Standard => {
                    let mut value = if ! self.into.is_disabled() {
                        quote!(#ident.into())
                    } else {
                        quote!(#ident)
                    };
                    if self.option.is_enabled() {
                        value = quote!(::core::option::Option::Some(#value));
                    }
                    if self.default.is_enabled() {
                        value = quote!(::macon::Defaulting::Set(#value));
                    }
                    if self.struct_default.is_enabled() {
                        value = quote!(::macon::Keeping::Set(#value));
                    }
                    value
                },
                Setter::None => {
                    let mut value = quote!(::core::option::Option::None);
                    if self.default.is_enabled() {
                        value = quote!(::macon::Defaulting::Set(#value));
                    }
                    if self.struct_default.is_enabled() {
                        value = quote!(::macon::Keeping::Set(#value));
                    }
                    value
                },
                Setter::Keep => quote!(::macon::Keeping::Keep),
                Setter::Default => {
                    let mut value = quote!(::macon::Defaulting::Default);
                    if self.struct_default.is_enabled() {
                        value = quote!(::macon::Keeping::Set(#value));
                    }
                    value
                },
            }
        } else {
            let id = self.id();
            quote!(self.#id)
        };
        quote!(#prefix #value,)
    }

    pub fn typestate_value(&self) -> TokenStream {
        let id = self.id();
        let mut value = quote!(self.#id);
        if self.struct_default.is_enabled() {
            value = quote!(#value.unwrap());
        }
        if self.default.is_enabled() {
            value = quote!(#value.unwrap());
        }
        value
    }

    pub fn typestate_build(&self) -> TokenStream {
        let prefix = self.prefix();
        let value = self.typestate_value();
        quote!(#prefix #value,)
    }

    pub fn typestate_override(&self) -> TokenStream {
        let id = self.id();
        let value = self.typestate_value();
        quote! {
            if self.#id.is_set() {
                built.#id = #value;
            }
        }
    }

    pub fn result_field(&self) -> TokenStream {
        let prefix = self.prefix();
        let mut ty = self.ty.to_token_stream();
        if ! self.is_required() {
            if self.default.is_enabled() {
                ty = quote!(::macon::Defaulting<#ty>);
            }
            if self.struct_default.is_enabled() {
                ty = quote!(::macon::Keeping<#ty>);
            }
        } else {
            ty = quote!(::macon::Building<#ty>);
        }
        quote!(#prefix #ty,)
    }

    pub fn result_assign(&self, setter: Setter) -> TokenStream {
        let ident = &self.ident;
        let id = self.id();
        let mut value = match setter {
            Setter::Standard => {
                let mut value = quote!(#ident);
                if ! self.into.is_disabled() {
                    value = quote!(#value.into());
                }
                if self.option.is_enabled() {
                    value = quote!(::core::option::Option::Some(#value));
                }
                value
            },
            Setter::None => quote!(::core::option::Option::None),
            Setter::Keep => quote!(::macon::Keeping::Keep),
            Setter::Default => quote!(::macon::Defaulting::Default),
        };
        if ! self.is_required() {
            if setter != Setter::Keep {
                if setter != Setter::Default {
                    if self.default.is_enabled() {
                        value = quote!(::macon::Defaulting::Set(#value));
                    }
                }
                if self.struct_default.is_enabled() {
                    value = quote!(::macon::Keeping::Set(#value));
                }
            }
        } else {
            value = quote!(::macon::Building::Set(#value));
        }
        quote!(self.#id = #value;)
    }

    pub fn result_value(&self) -> TokenStream {
        let id = self.id();
        let mut value = quote!(self.#id);
        if ! self.is_required() {
            if self.struct_default.is_enabled() {
                value = quote!(#value.unwrap());
            }
            if self.default.is_enabled() {
                value = quote!(#value.unwrap());
            }
        } else {
            value = quote!(#value.unwrap());
        }
        value
    }

    pub fn result_build(&self) -> TokenStream {
        let prefix = self.prefix();
        let value = self.result_value();
        quote!(#prefix #value,)
    }

    pub fn result_override(&self) -> TokenStream {
        let id = self.id();
        let value = self.result_value();
        quote! {
            if self.#id.is_set() {
                built.#id = #value;
            }
        }
    }

}

impl Properties {
    pub fn delimiter(&self) -> Delimiter {
        if self.is_tuple {
            Delimiter::Parenthesis
        } else {
            Delimiter::Brace
        }
    }

    pub fn group(&self, stream: TokenStream) -> TokenStream {
        Group::new(self.delimiter(), stream).to_token_stream()
    }

    pub fn typestate_default(&self) -> TokenStream {
        self.to_token(|f| {
            let typestate = f.typevar();
            quote!(#typestate=(),)
        })
    }

    pub fn typestate_struct_fields(&self) -> TokenStream {
        let data_fields = self.to_token(|p| p.typestate_struct_field());
        let marker_field = self.typestate_marker_field(self.typestate_optional_marker());
        self.group(quote! {
            #data_fields
            #marker_field
        })
    }

    pub fn typestate_marker_field(&self, typestate: TokenStream) -> TokenStream {
        if self.is_tuple {
            quote!(::core::marker::PhantomData<(#typestate)>,)
        } else {
            quote!(__typestate_markers: ::core::marker::PhantomData<(#typestate)>,)
        }
    }

    pub fn typestate_optional_marker(&self) -> TokenStream {
        self.to_token(|p| p.typestate_optional_marker())
    }

    pub fn typestate_state(&self, target: &Property, is_ordered: bool, is_set: bool) -> TokenStream {
        self.to_token(|p| p.typestate_state(target, is_ordered, is_set))
    }

    pub fn typestate_state_final(&self) -> TokenStream {
        self.to_token(|p| p.typestate_state_final())
    }

    pub fn typestate_setter_impl(&self, target: &Property) -> TokenStream {
        self.to_token(|p| p.typestate_setter_impl(target))
    }

    pub fn typestate_assign(&self, target: &Property, setter: Setter) -> TokenStream {
        let data = self.to_token(|p| p.typestate_assign(target, setter));
        let marker = if self.is_tuple {
            quote!(::core::default::Default::default(),)
        } else {
            quote!(__typestate_markers: ::core::default::Default::default(),)
        };
        self.group(quote! {
            #data
            #marker
        })
    }

    pub fn typestate_build(&self) -> TokenStream {
        self.group(self.to_token(|p| p.typestate_build()))
    }

    pub fn typestate_override(&self) -> TokenStream {
        self.to_token(|p| p.typestate_override())
    }

    pub fn result_fields(&self) -> TokenStream {
        self.group(self.to_token(|p| p.result_field()))
    }

    pub fn result_build(&self) -> TokenStream {
        self.group(self.to_token(|p| p.result_build()))
    }

    pub fn result_override(&self) -> TokenStream {
        self.to_token(|p| p.result_override())
    }
}

#[cfg(test)]
pub mod tests {
    use proc_macro2::Span;
    use syn::{
        parse_quote,
        parse_str,
    };

    use super::*;

    fn newbuilder(derive: DeriveInput) -> Builder {
        Builder::from_input(derive).expect("Builder::from_input")
    }

    fn errbuilder(derive: DeriveInput) -> ::std::result::Result<(), String> {
        Builder::from_input(derive)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    #[test]
    fn builder_attribute_mode_panic() {
        let builder = newbuilder(parse_quote! {
            #[builder(mode=Panic)]
            struct Foobar;
        });
        assert_eq!(
            builder.mode,
            Mode::Panic,
        );
    }

    #[test]
    fn builder_attribute_mode_default() {
        let builder = newbuilder(parse_quote! {
            #[builder]
            struct Foobar;
        });
        assert_eq!(
            builder.mode,
            Mode::Typestate,
        );
    }

    #[test]
    fn builder_attribute_into() {
        let builder = newbuilder(parse_quote! {
            #[builder]
            struct Foobar;
        });
        assert_eq!(
            builder.properties.into,
            Setting::undefined(),
        );
    }

    #[test]
    fn builder_attribute_into_disabled() {
        let builder = newbuilder(parse_quote! {
            #[builder(Into=!)]
            struct Foobar;
        });
        assert_eq!(
            builder.properties.into,
            Setting::disable(Span::call_site()),
        );
    }

    #[test]
    fn builder_attribute_mode_unknown() {
        let actual = errbuilder(parse_quote! {
            #[builder(mode=Unknown)]
            struct Foobar;
        });
        assert_eq!(
            actual,
            std::result::Result::Err("Unsupported mode value Unknown for struct builder attribute".to_owned()),
        );
    }

    #[test]
    fn builder_derive_properties() {
        let builder = newbuilder(parse_quote! {
            #[derive(Builder)]
            struct Foobar {
                foo: i32,
                bar: String,
            }
        });
        let actual: Vec<_> = builder.properties.iter().map(|p| &p.name).collect();
        let expected = vec![
                "foo",
                "bar",
        ];
        assert_eq!(
            actual,
            expected,
        )
    }

    #[test]
    fn builder_derive_option() {
        let builder = newbuilder(parse_quote! {
            #[derive(Builder)]
            struct WithOption {
                optional: Option<String>,
            }
        });
        assert_eq!(builder.ident, format_ident!("WithOptionBuilder"), "builder.ident");
        assert_eq!(builder.target, format_ident!("WithOption"), "builder.target");

        let mut iter = builder.properties.items.into_iter();

        let optional = iter.next().expect("builder.properties[0]");
        assert_eq!(optional.ident, format_ident!("optional"));
        assert_eq!(
            optional.option.value().map(|t| t.to_token_stream().to_string()),
            Some(String::from("String")),
            "builder.properties[0].option"
        );
    }

    #[test]
    fn builder_derive_option_disable_at_struct() {
        let builder = newbuilder(parse_quote! {
            #[derive(Builder)]
            #[builder(Option=!,)]
            struct DisabledOptionAtStruct {
                optional0: Option<String>,
                optional1: Option<String>,
            }
        });
        assert_eq!(builder.ident, format_ident!("DisabledOptionAtStructBuilder"), "builder.ident");
        assert_eq!(builder.target, format_ident!("DisabledOptionAtStruct"), "builder.target");

        let mut iter = builder.properties.items.into_iter();

        let optional0 = iter.next().expect("builder.properties[0]");
        assert_eq!(optional0.ident, format_ident!("optional0"), "builder.properties[0]");
        assert!(
            optional0.option.is_disabled(),
            "builder.properties[0].option.disabled"
        );
        assert_eq!(
            optional0.ty,
            parse_str::<Type>("Option<String>").unwrap(),
            "builder.properties[0].ty"
        );
    }

    #[test]
    fn builder_derive_default_unspecified() {
        let builder = newbuilder(parse_quote! {
            #[derive(Builder,)]
            struct Demo;
        });

        assert_eq!(builder.default, Setting::undefined(), "builder.default");
    }

    #[test]
    fn builder_derive_default_derive() {
        let builder = newbuilder(parse_quote! {
            #[derive(Builder,Default,)]
            struct Demo;
        });

        assert_eq!(builder.default, Setting::enable((), Span::call_site()), "builder.default");
    }

    #[test]
    fn builder_derive_default_option() {
        let builder = newbuilder(parse_quote! {
            #[derive(Builder,)]
            #[builder(Default,)]
            struct Demo;
        });

        assert_eq!(builder.default, Setting::enable((), Span::call_site()), "builder.default")
    }

    #[test]
    fn builder_derive_default_disabled() {
        let builder = newbuilder(parse_quote! {
            #[derive(Builder,Default,)]
            #[builder(Default=!,)]
            struct Demo;
        });

        assert_eq!(builder.default, Setting::disable(Span::call_site()), "builder.default")
    }
}
