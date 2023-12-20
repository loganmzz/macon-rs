use proc_macro2::{
    Literal,
    TokenStream,
};
use quote::{format_ident, quote};
use syn::{
    Attribute,
    Data,
    DeriveInput,
    Error,
    Field,
    Fields,
    Ident,
    Meta,
    MetaList,
    Result,
    Type,
};

#[derive(Debug,PartialEq)]
pub struct Builder {
    pub ident: Ident,
    pub target: Ident,
    pub mode: Mode,
    pub properties: Vec<Property>,
    pub is_tuple: bool,
}

#[derive(Debug,PartialEq)]
pub enum Mode {
    Typestate,
    Result,
    Panic,
}

#[derive(Debug,PartialEq)]
pub struct Property {
    pub ordinal: usize,
    pub name: String,
    pub ident: Ident,
    pub ty: Type,
    pub is_tuple: bool,
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
            mode: Default::default(),
            properties: Default::default(),
            is_tuple: false,
        }
    }
}

impl Builder {
    pub fn from_input(derive: DeriveInput) -> Result<Self> {
        let mut this = Self::default();
        this.target = derive.ident;
        this.ident = format_ident!("{}Builder", this.target);
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
                            self.properties.push(Property::from_field(false, ordinal, field)?);
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
                            self.properties.push(Property::from_field(true, ordinal, field)?);
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
    pub fn from_field(is_tuple: bool, ordinal: usize, field: Field) -> Result<Self> {
        let ident = field.ident.unwrap_or_else(|| format_ident!("v{}", ordinal));
        let name = ident.to_string();
        Ok(Self {
            ordinal,
            name,
            ident,
            ty: field.ty,
            is_tuple,
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
}

#[cfg(test)]
pub mod tests {
    use syn::{
        parse_quote,
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
}