use ::std::borrow::Cow;

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
use syn::{
    AngleBracketedGenericArguments,
    Attribute,
    Data,
    DeriveInput,
    Error,
    Field,
    Fields,
    GenericArgument,
    Ident,
    Meta,
    MetaList,
    PathArguments,
    Result,
    Token,
    Type,
    Visibility,
};

#[derive(Debug)]
pub struct Builder {
    pub ident: Ident,
    pub target: Ident,
    pub vis: Visibility,
    pub mode: Mode,
    pub properties: Properties,
    pub is_tuple: bool,
    pub option: bool,
    pub default: Setting<()>,
}

#[derive(Debug,PartialEq)]
pub enum Mode {
    Typestate,
    Result,
    Panic,
}

#[derive(Debug)]
pub enum Setting<T> {
    Undefined,
    Disabled,
    Enabled(T),
}

impl<T: PartialEq> PartialEq for Setting<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Undefined, Self::Undefined) => true,
            (Self::Disabled, Self::Disabled) => true,
            (Self::Enabled(self_value), Self::Enabled(other_value)) => self_value == other_value,
            _ => false,
        }
    }
}
impl<T: Clone> Clone for Setting<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Undefined => Self::Undefined,
            Self::Disabled => Self::Disabled,
            Self::Enabled(value) => Self::Enabled(value.clone()),
        }
    }
}

impl<T> Setting<T> {
    pub fn undefine() -> Self {
        Setting::Undefined
    }
    pub fn disable() -> Self {
        Setting::Disabled
    }
    pub fn enable(value: T) -> Self {
        Setting::Enabled(value)
    }

    pub fn is_defined(&self) -> bool {
        !self.is_undefined()
    }

    pub fn is_undefined(&self) -> bool {
        matches!(self, Self::Undefined)
    }

    pub fn is_disabled(&self) -> bool {
        matches!(self, Self::Disabled)
    }

    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled(_))
    }

    pub fn value(&self) -> Option<&T> {
        match self {
            Self::Enabled(ref value) => Some(value),
            _ => None,
        }
    }
}

impl<T> Default for Setting<T> {
    fn default() -> Self {
        Self::undefine()
    }
}

impl<T> From<T> for Setting<T> {
    fn from(value: T) -> Self {
        Self::enable(value)
    }
}

#[derive(Debug,Default)]
pub struct PropertySettings {
    pub option: Setting<Type>,
}

#[derive(Debug,Default,)]
pub struct Properties(Vec<Property>);
impl ::core::ops::Deref for Properties {
    type Target = Vec<Property>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ::core::ops::DerefMut for Properties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
    pub ordinal: usize,
    pub name: String,
    pub ident: Ident,
    pub ty: Type,
    pub is_tuple: bool,
    pub option: Setting<Type>,
    pub default: Setting<()>,
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
            option: true,
            default: Default::default(),
        }
    }
}

trait ResultErrorContext {
    fn map_err_context<C: ::core::fmt::Display>(self, context: C) -> Self;
}
impl<T> ResultErrorContext for Result<T> {
    fn map_err_context<C: ::core::fmt::Display>(self, context: C) -> Self {
        if let Err(err) = self {
            let mut newerr: Option<Error> = None;
            for e in err {
                let contextualized = Error::new(
                    e.span(),
                    format!("{}: {}", context, e),
                );
                if let Some(ref mut error) = newerr {
                    error.extend(vec![contextualized]);
                } else {
                    newerr = Some(contextualized);
                }
            }
            Err(newerr.unwrap())
        } else {
            self
        }
    }
}

impl Builder {
    pub fn from_input(derive: DeriveInput) -> Result<Self> {
        let mut this = Self::default();
        this.target = derive.ident;
        this.ident = format_ident!("{}Builder", this.target);
        this.vis = derive.vis;
        for attr in derive.attrs {
            this.with_attribute(attr)?;
        }
        this.with_data(derive.data)?;
        Ok(this)
    }

    pub fn with_attribute(&mut self, attr: Attribute) -> Result<()> {
        if let Meta::List(meta_list) = attr.meta {
            if meta_list.path.is_ident("builder") {
                self.with_builder_attribute(meta_list)?;
            } else if meta_list.path.is_ident("derive") {
                self.with_derive_attribute(meta_list)?;
            }
        }
        Ok(())
    }

    pub fn with_derive_attribute(&mut self, meta_list: MetaList) -> Result<()> {
        meta_list.parse_nested_meta(|nested| {
            if nested.path.is_ident("Default") {
                if ! self.default.is_defined() {
                    self.default = Setting::enable(());
                }
            }
            Ok(())
        })
    }

    pub fn with_builder_attribute(&mut self, meta_list: MetaList) -> Result<()> {
        meta_list.parse_nested_meta(|nested| {
            if nested.path.is_ident("mode") {
                let value: Ident = nested
                    .value()
                    .map_err_context("Unable to parse mode value for struct builder attribute")?
                    .parse()
                    .map_err_context("Unable to parse into Ident mode value for struct builder attribute")?;
                if value == "Typestate" {
                    self.mode = Mode::Typestate;
                } else if value == "Result" {
                    self.mode = Mode::Result;
                } else if value == "Panic" {
                    self.mode = Mode::Panic;
                } else {
                    return Err(nested.error(format!("Unsupported mode value {} for struct builder attribute", value)));
                }
                Ok(())
            } else if nested.path.is_ident("Option") {
                let value: Type = nested
                    .value()
                    .map_err_context("Unable to parse Option value for struct builder attribute")?
                    .parse()
                    .map_err_context("Unable to parse into Option value for struct builder attribute")?;
                if let Type::Never(_) = value {
                    self.option = false;
                    Ok(())
                } else {
                    Err(nested.error(format!("Unsupported Option value {:?} for struct builder attribute", value)))
                }
            } else if nested.path.is_ident("Default") {
                if nested.input.is_empty() || nested.input.peek(Token![,]) {
                    self.default = Setting::enable(());
                    Ok(())
                } else if nested.input.peek(Token![=]) {
                    let value: Type = nested
                        .value()
                        .map_err_context("Unable to parse Default value for struct builder attribute")?
                        .parse()
                        .map_err_context("Unable to parse Default value into Type for struct builder attribute")?;
                    if let Type::Never(_) = value {
                        self.default = Setting::disable();
                        Ok(())
                    } else {
                        Err(nested.error(format!("Unsupported Default value {:?} for struct builder attribute", value)))
                    }
                } else {
                    Err(nested.error(format!("Unsupported Default format {:?} for struct builder attribute", nested.input)))
                }
            } else {
                Err(nested.error(format!("Unsupported struct builder attribute option: {:?}", nested.path)))
            }
        })
        .map_err_context("Unable to read struct builder attribute")
    }

    pub fn with_data(&mut self, data: Data) -> Result<()> {
        match data {
            Data::Struct(data_struct) => {
                match data_struct.fields {
                    Fields::Named(fields_named) => {
                        self.is_tuple = false;
                        self.properties = Default::default();
                        for (ordinal, field) in fields_named.named.into_iter().enumerate() {
                            let value = Property::from_field(self, false, ordinal, field)?;
                            self.properties.push(value);
                        }
                        Ok(())
                    },
                    Fields::Unit => {
                        self.properties = Default::default();
                        Ok(())
                    },
                    Fields::Unnamed(fields_unamed) => {
                        self.is_tuple = true;
                        self.properties = Default::default();
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
}

impl Property {
    pub fn from_field(builder: &Builder, is_tuple: bool, ordinal: usize, field: Field) -> Result<Self> {
        let ident = field.ident.unwrap_or_else(|| format_ident!("v{}", ordinal));
        let name = ident.to_string();
        let mut settings = PropertySettings::default();
        if !builder.option {
            settings.option = Setting::disable();
        }
        for metalist in field.attrs
            .iter()
            .filter_map(|attr|
                if let Meta::List(ref meta_list) = attr.meta {
                    if meta_list.path.is_ident("builder") {
                        Some(meta_list)
                    } else {
                        None
                    }
                } else {
                    None
                }
            ) {
                metalist.parse_nested_meta(|nested| {
                    if nested.path.is_ident("Option") {
                        if settings.option.is_defined() {
                            return Err(nested.error(format!("Option has been already specified: {:?}", settings.option)));
                        }
                        let type_: Type = nested.value()?.parse()?;
                        settings.option = match type_ {
                            Type::Tuple(ref typetuple) => {
                                if typetuple.elems.is_empty() {
                                    Setting::enable(type_)
                                } else {
                                    Setting::disable()
                                }
                            },
                            Type::Never(_) => Setting::disable(),
                            _ => Setting::enable(type_),
                        }
                    } else {
                        return Err(nested.error(format!("Unsupported option {:?}", nested.path)));
                    }
                    Ok(())
                })?;
            }
        if settings.option.is_undefined() {
            let (path, generic_args) = Self::read_path(&field.ty);
            match path.as_str() {
                "std::option::Option"|"core::option::Option"|"Option" => {
                    if let Some(generic_args) = generic_args {
                        if generic_args.args.len() == 1 {
                            if let Some(GenericArgument::Type(type_)) = generic_args.args.first() {
                                settings.option = Setting::enable(type_.clone());
                            }
                        }
                    }
                },
                _ => {},
            };
        }
        Ok(Self {
            ordinal,
            name,
            ident,
            ty: field.ty,
            is_tuple,
            option: settings.option,
            default: builder.default.clone(),
        })
    }

    pub fn read_path(ty: &Type) -> (String, Option<&AngleBracketedGenericArguments>) {
        match ty {
            Type::Path(typepath) => {
                let path = typepath.path.segments
                    .iter()
                    .map(|pathsegment| pathsegment.ident.to_string())
                    .collect::<Vec<_>>()
                    .join("::");
                let generic_args = typepath.path.segments.last().and_then(|s| match s.arguments {
                    PathArguments::None => None,
                    PathArguments::Parenthesized(_) => None,
                    PathArguments::AngleBracketed(ref args) => Some(args),
                });
                (path, generic_args)
            },
            _ => {
                ("".into(), None,)
            }
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

    pub fn prefix(&self) -> TokenStream {
        if self.is_tuple {
            quote!()
        } else {
            let ident = &self.ident;
            quote!(#ident:)
        }
    }

    pub fn assign(&self, target: &Property, is_none: bool, is_default: bool) -> TokenStream {
        let prefix = self.prefix();
        let value = if self.name == target.name {
            if is_default {
                quote!(::core::option::Option::None)
            } else {
                let mut value = if is_none {
                    quote!(::core::option::Option::None)
                } else {
                    let ident = &self.ident;
                    if self.option.is_enabled() {
                        quote!(::core::option::Option::Some(#ident.into()))
                    } else {
                        quote!(#ident.into())
                    }
                };
                if self.default.is_enabled() {
                    value = quote!(::core::option::Option::Some(#value));
                }
                value
            }
        } else {
            let id = self.id();
            quote!(self.#id)
        };
        quote!(#prefix #value,)
    }

    pub fn build_overrides(&self) -> TokenStream {
        let ident = &self.ident;
        let id = self.id();
        quote! {
            if let ::core::option::Option::Some(#ident) = self.#id {
                built.#id = #ident;
            }
        }
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

    pub fn typestate_init(&self) -> TokenStream {
        let prefix = self.prefix();
        let mut ty = if self.option.is_enabled() || self.default.is_enabled() {
            self.ty.to_token_stream()
        } else {
            self.typevar().to_token_stream()
        };
        if self.default.is_enabled() {
            ty = quote!(::core::option::Option<#ty>);
        }
        quote!(#prefix #ty,)
    }

    pub fn typestate_optional_marker(&self) -> TokenStream {
        if self.option.is_enabled() || self.default.is_enabled() {
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
        if self.option.is_enabled() || self.default.is_enabled() {
            self.typestate(Some(true))
        } else {
            self.typestate(Some(false))
        }
    }

    pub fn typestate_setter_impl(&self, target: &Property) -> TokenStream {
        if self.name == target.name {
            quote!()
        } else {
            self.typestate(Some(true))
        }
    }

    pub fn typestate_build(&self) -> TokenStream {
        let id = self.id();
        let prefix = self.prefix();
        quote!(#prefix self.#id,)
    }

    pub fn result_field(&self) -> TokenStream {
        let prefix = self.prefix();
        let ty = if self.default.is_enabled() {
            &self.ty
        } else {
            self.ty_into()
        };
        quote!(#prefix ::core::option::Option<#ty>,)
    }

    pub fn result_build(&self) -> TokenStream {
        let prefix = self.prefix();
        let id = self.id();
        if self.option.is_enabled() {
            quote!(#prefix self.#id,)
        } else {
            quote!(#prefix self.#id.unwrap(),)
        }
    }
}

impl Properties {
    pub fn typestate_default(&self) -> TokenStream {
        self.to_token(|f| {
            let typestate = f.typevar();
            quote!(#typestate=(),)
        })
    }

    pub fn typestate_init(&self) -> TokenStream {
        self.to_token(|p| p.typestate_init())
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

    pub fn typestate_build(&self) -> TokenStream {
        self.to_token(|p| p.typestate_build())
    }

    pub fn typestate_build_overrides(&self) -> TokenStream {
        self.to_token(|p| p.build_overrides())
    }

    pub fn result_fields(&self) -> TokenStream {
        self.to_token(|p| p.result_field())
    }

    pub fn result_build(&self) -> TokenStream {
        self.to_token(|p| p.result_build())
    }

    pub fn assign(&self, target: &Property, is_none: bool, is_default: bool) -> TokenStream {
        self.to_token(|p| p.assign(target, is_none, is_default))
    }
}

#[cfg(test)]
pub mod tests {
    use quote::ToTokens;
    use syn::{
        parse_quote,
        parse_str,
        Attribute,
    };

    use super::*;

    fn newbuilder(derive: DeriveInput) -> Builder {
        eprintln!("{:#?}", derive);
        Builder::from_input(derive).expect("Buider::from_input")
    }

    #[test]
    fn builder_attribute_mode_panic() {
        let mut builder = Builder::default();
        let attribute: Attribute = parse_quote! {
            #[builder(mode=Panic)]
        };
        builder.with_attribute(attribute).unwrap();
        assert_eq!(
            builder.mode,
            Mode::Panic,
        );
    }

    #[test]
    fn builder_attribute_mode_default() {
        let mut builder = Builder::default();
        let attribute: Attribute = parse_quote! {
            #[builder]
        };
        builder.with_attribute(attribute).unwrap();
        assert_eq!(
            builder.mode,
            Mode::Typestate,
        );
    }

    #[test]
    fn builder_attribute_mode_unknown() {
        let mut builder = Builder::default();
        let attribute: Attribute = parse_quote! {
            #[builder(mode=Unknown)]
        };
        let actual = builder.with_attribute(attribute).map_err(|e| e.to_string());
        assert_eq!(
            actual,
            std::result::Result::Err("Unable to read struct builder attribute: Unsupported mode value Unknown for struct builder attribute".to_owned()),
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

        let mut iter = builder.properties.0.into_iter();

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

        let mut iter = builder.properties.0.into_iter();

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

        assert_eq!(builder.default, Setting::undefine(), "builder.default");
    }

    #[test]
    fn builder_derive_default_derive() {
        let builder = newbuilder(parse_quote! {
            #[derive(Builder,Default,)]
            struct Demo;
        });

        assert_eq!(builder.default, Setting::enable(()), "builder.default");
    }

    #[test]
    fn builder_derive_default_option() {
        let builder = newbuilder(parse_quote! {
            #[derive(Builder,)]
            #[builder(Default,)]
            struct Demo;
        });

        assert_eq!(builder.default, Setting::enable(()), "builder.default")
    }

    #[test]
    fn builder_derive_default_disabled() {
        let builder = newbuilder(parse_quote! {
            #[derive(Builder,Default,)]
            #[builder(Default=!,)]
            struct Demo;
        });

        assert_eq!(builder.default, Setting::disable(), "builder.default")
    }

    #[test]
    fn property_read_path() {
        let type_ = parse_quote!(MyStruct);
        let (path, args) = Property::read_path(&type_);
        assert_eq!(&path, "MyStruct");
        assert_eq!(args, None);
    }
}
