use proc_macro2::{
    Literal,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
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
    Type,
    Visibility,
};

#[derive(Debug)]
pub struct Builder {
    pub ident: Ident,
    pub target: Ident,
    pub vis: Visibility,
    pub mode: Mode,
    pub properties: Vec<Property>,
    pub is_tuple: bool,
    pub option: bool,
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
        match self {
            Self::Undefined => true,
            _ => false,
        }
    }

    pub fn is_disabled(&self) -> bool {
        match self {
            Self::Disabled => true,
            _ => false,
        }
    }

    pub fn is_enabled(&self) -> bool {
        match self {
            Self::Enabled(_) => true,
            _ => false,
        }
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

#[derive(Debug)]
pub struct Property {
    pub ordinal: usize,
    pub name: String,
    pub ident: Ident,
    pub ty: Type,
    pub is_tuple: bool,
    pub option: Setting<Type>,
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
                self.with_metalist(meta_list)?;
            }
        }
        Ok(())
    }

    pub fn with_metalist(&mut self, meta_list: MetaList) -> Result<()> {
        meta_list.parse_nested_meta(|nested| {
            if nested.path.is_ident("mode") {
                let value: Ident = nested.value()?.parse()?;
                if value == "Typestate" {
                    self.mode = Mode::Typestate;
                } else if value == "Result" {
                    self.mode = Mode::Result;
                } else if value == "Panic" {
                    self.mode = Mode::Panic;
                } else {
                    return Err(nested.error(format!("Unsupported value {} for mode", value)));
                }
                Ok(())
            } else if nested.path.is_ident("Option") {
                let value: Type = nested.value()?.parse()?;
                if let Type::Never(_) = value {
                    self.option = false;
                    Ok(())
                } else {
                    Err(nested.error(format!("Unsupported Option value for struct: {:?}", value)))
                }
            } else {
                Err(nested.error(format!("Unsupported builder option: {:?}", nested.path)))
            }
        })
    }

    pub fn with_data(&mut self, data: Data) -> Result<()> {
        match data {
            Data::Struct(data_struct) => {
                match data_struct.fields {
                    Fields::Named(fields_named) => {
                        self.is_tuple = false;
                        self.properties = Vec::default();
                        for (ordinal, field) in fields_named.named.into_iter().enumerate() {
                            self.properties.push(Property::from_field(self, false, ordinal, field)?);
                        }
                        Ok(())
                    },
                    Fields::Unit => {
                        self.properties = Vec::default();
                        Ok(())
                    },
                    Fields::Unnamed(fields_unamed) => {
                        self.is_tuple = true;
                        self.properties = Vec::default();
                        for (ordinal, field)  in fields_unamed.unnamed.into_iter().enumerate() {
                            self.properties.push(Property::from_field(self, true, ordinal, field)?);
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

    #[test]
    pub fn builder_attribute_mode_panic() {
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
    pub fn builder_attribute_mode_default() {
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
    pub fn builder_attribute_mode_unknown() {
        let mut builder = Builder::default();
        let attribute: Attribute = parse_quote! {
            #[builder(mode=Unknown)]
        };
        let actual = builder.with_attribute(attribute).map_err(|e| e.to_string());
        assert_eq!(
            actual,
            std::result::Result::Err("Unsupported value Unknown for mode".to_owned()),
        );
    }

    #[test]
    pub fn builder_derive_properties() {
        let derive: DeriveInput = parse_quote! {
            #[derive(Builder)]
            struct Foobar {
                foo: i32,
                bar: String,
            }
        };
        let builder = Builder::from_input(derive).unwrap();
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
    pub fn builder_derive_option() {
        let derive: DeriveInput = parse_quote! {
            #[derive(Builder)]
            struct WithOption {
                optional: Option<String>,
            }
        };
        let builder = Builder::from_input(derive).unwrap();
        assert_eq!(builder.ident, format_ident!("WithOptionBuilder"), "builder.ident");
        assert_eq!(builder.target, format_ident!("WithOption"), "builder.target");

        let mut iter = builder.properties.into_iter();

        let optional = iter.next().expect("builder.properties[0]");
        assert_eq!(optional.ident, format_ident!("optional"));
        assert_eq!(
            optional.option.value().map(|t| t.to_token_stream().to_string()),
            Some(String::from("String")),
            "builder.properties[0].option"
        );
    }

    #[test]
    pub fn builder_derive_option_disable_at_struct() {
        let derive: DeriveInput = parse_quote! {
            #[derive(Builder)]
            #[builder(Option=!)]
            struct DisabledOptionAtStruct {
                optional0: Option<String>,
                optional1: Option<String>,
            }
        };
        let builder = Builder::from_input(derive).unwrap();
        assert_eq!(builder.ident, format_ident!("DisabledOptionAtStructBuilder"), "builder.ident");
        assert_eq!(builder.target, format_ident!("DisabledOptionAtStruct"), "builder.target");

        let mut iter = builder.properties.into_iter();

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
    pub fn property_read_path() {
        let type_ = parse_quote!(MyStruct);
        let (path, args) = Property::read_path(&type_);
        assert_eq!(&path, "MyStruct");
        assert_eq!(args, None);
    }

    // #[test]
    // pub fn property_read_path2() {
    //     fn assert_ident(actual: Type, expected: &'static str) {
    //         if let Type::Path(typepath) = actual {
    //             assert!(typepath.path.is_ident(expected), "expected={}", expected);
    //         } else {
    //             panic!("not a path");
    //         }
    //     }
    //     for (actual,expected) in [
    //         (parse_quote!(MyStruct), "MyStruct"),
    //         (parse_quote!(std::option::Option), "Option"),
    //         (parse_quote!(::std::option::Option), "::std::Option"),
    //     ] {
    //         assert_ident(actual, expected);
    //     }
    // }

    // #[test]
    // pub fn property_read_attrs() {
    //     let attr: Attribute = parse_quote!(#[builder(mode=Panic,defaults(Option))]);
    //     println!("{:#?}", attr);
    //     panic!("");
    // }
}