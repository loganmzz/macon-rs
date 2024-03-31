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
use syn::{
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

#[derive(Clone,Copy,Debug,PartialEq,)]
pub enum Setter {
    Standard,
    None,
    Keep,
    Default,
}

impl<T: Copy> Copy for Setting<T> {
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
    pub default: Setting<()>,
}

#[derive(Debug,Default,)]
pub struct Properties {
    pub is_tuple: bool,
    pub default: Setting<()>,
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
    pub ordinal: usize,
    pub name: String,
    pub ident: Ident,
    pub ty: Type,
    pub is_tuple: bool,
    pub option: Setting<Type>,
    pub default: Setting<()>,
    pub struct_default: Setting<()>,
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
                    self.properties.default = Setting::enable(());
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
                    self.set_default(Setting::enable(()));
                    Ok(())
                } else if nested.input.peek(Token![=]) {
                    let value: Type = nested
                        .value()
                        .map_err_context("Unable to parse Default value for struct builder attribute")?
                        .parse()
                        .map_err_context("Unable to parse Default value into Type for struct builder attribute")?;
                    if let Type::Never(_) = value {
                        self.set_default(Setting::disable());
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
                        };
                    } else if nested.path.is_ident("Default") {
                        if settings.default.is_defined() {
                            return Err(nested.error(format!("Default has been already specified: {:?}", settings.default)));
                        }
                        if nested.input.is_empty() || nested.input.peek(Token![,]) {
                            settings.default = Setting::enable(());
                        } else if nested.input.peek(Token![=]) {
                            let value: Type = nested
                                .value()
                                .map_err_context(format!("Unable to parse Default value for field ({:?}) builder attribute", name))?
                                .parse()
                                .map_err_context(format!("Unable to parse Default value into Type for field ({:?}) builder attribute", name))?;
                            if let Type::Never(_) = value {
                                settings.default = Setting::disable();
                            } else {
                                return Err(nested.error(format!("Unsupported Default value {:?} for field ({:?}) builder attribute", value, name)));
                            }
                        } else {
                            return Err(nested.error(format!("Unsupported Default format {:?} for field ({:?}) builder attribute", nested.input, name)));
                        }
                    } else {
                        return Err(nested.error(format!("Unsupported option {:?} for field ({:?}) builder attribute", nested.path, name)));
                    }
                    Ok(())
                })?;
            }
        if settings.option.is_undefined() {
            if let Some(ty) = Self::get_option_arg(&field.ty)? {
                settings.option = Setting::enable(ty.clone());
            }
        }
        if settings.default.is_undefined() {
            let default_types = match crate::config::get() {
                Ok(config) => config.default_types(),
                Err(err) => return Err(Error::new_spanned(&field, err)),
            };
            if default_types.match_type(&field.ty) {
                settings.default = Setting::Enabled(());
            }
        }
        Ok(Self {
            ordinal,
            name,
            ident,
            ty: field.ty,
            is_tuple,
            option: settings.option,
            default: settings.default,
            struct_default: builder.default.clone(),
        })
    }

    pub fn get_option_arg(ty: &Type) -> Result<Option<&Type>> {
        let config = match crate::config::get() {
            Ok(config) => config,
            Err(error) => return Err(Error::new_spanned(ty, error)),
        };
        Ok(if config.option_types().match_type(ty) {
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
        })
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
                    let mut value = quote!(#ident.into());
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
                let mut value = quote!(#ident.into());
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
}
