//! Generator panicing on missing fields and ignore setting many times.

use super::*;

pub struct ResultPanicGenerator {
    builder: Builder,
}

impl ResultPanicGenerator {
    pub fn new(builder: Builder) -> Self {
        Self {
            builder,
        }
    }
}

impl ResultPanicGenerator {

    /// Generate builder struct
    pub fn struct_builder(&self) -> TokenStream {
        let vis = &self.builder.vis;
        let fields = self.builder.group(self.properties().result_fields());
        let builder_name = &self.builder.ident;
        let delim = if self.builder.is_tuple {
            quote!(;)
        } else {
            quote!()
        };
        quote! {
            #[derive(Default)]
            #vis struct #builder_name #fields #delim
        }
    }

    /// Generate `impl` block for generated builder struct:
    ///
    /// * fluent field setters ([`Self::impl_builder_setters()`])
    /// * final `build()` function ([`Self::impl_builder_build()`])
    pub fn impl_builder(&self) -> TokenStream {
        let builder_name = &self.builder.ident;
        let impl_builder_setters = self.impl_builder_setters();
        let impl_builder_build = self.impl_builder_build();
        let impl_builder_from = self.impl_builder_from();
        quote! {
            impl #builder_name {
                #impl_builder_setters
                #impl_builder_build
            }
            #impl_builder_from
        }
    }

    /// Generate fluent field setters
    pub fn impl_builder_setters(&self) -> TokenStream {
        self.properties().to_token(|f| {
            let method = f.setter();
            let typevar = f.typevar();
            let id = f.id();
            let ty = f.ty_into();
            let ident = &f.ident;
            let mut value = quote!(::core::option::Option::Some(#ident.into()));
            if f.default.is_enabled() && f.option.is_enabled() {
                value = quote!(::core::option::Option::Some(#value));
            }
            let setter_none = if f.option.is_enabled() {
                let setter_none = f.setter_none();
                let mut value_none = quote!(::core::option::Option::None);
                if f.default.is_enabled() && f.option.is_enabled() {
                    value_none = quote!(::core::option::Option::Some(#value_none));
                }
                quote! {
                    pub fn #setter_none(mut self) -> Self {
                        self.#id = #value_none;
                        self
                    }
                }
            } else {
                quote!()
            };
            let setter_keep = if f.default.is_enabled() {
                let setter_keep = f.setter_keep();
                quote! {
                    pub fn #setter_keep(mut self) -> Self {
                        self.#id = ::core::option::Option::None;
                        self
                    }
                }
            } else {
                quote!()
            };
            quote! {
                pub fn #method<#typevar: ::core::convert::Into<#ty>>(mut self, #ident: #typevar) -> Self {
                    self.#id = #value;
                    self
                }
                #setter_none
                #setter_keep
            }
        })
    }

    /// Generate final `build()` function
    pub fn impl_builder_build(&self) -> TokenStream {
        let target = &self.builder.target;
        let output = match self.builder.mode {
            Mode::Panic => quote!(#target),
            Mode::Result => quote!(::core::result::Result<#target, ::std::string::String>),
            _ => panic!("Unsupported mode {:?}", self.builder.mode),
        };
        let content = if self.builder.default.is_enabled() {
            self.impl_builder_build_from_default()
        } else {
            self.impl_builder_build_from_scratch()
        };
        quote! {
            pub fn build(self) -> #output {
                #content
            }
        }
    }

    pub fn impl_builder_build_from_default(&self) -> TokenStream {
        let target = &self.builder.target;
        let overrides = self.properties().typestate_build_overrides();
        let mut result = quote!(built);
        if self.builder.mode == Mode::Result {
            result = quote!(::core::result::Result::Ok(#result))
        }
        quote! {
            let mut built = <#target as ::core::default::Default>::default();
            #overrides
            #result
        }
    }

    pub fn impl_builder_build_from_scratch(&self) -> TokenStream {
        let target = &self.builder.target;
        let check_fields = self.properties().to_token(|f| {
            if f.option.is_enabled() {
                return quote!();
            }
            let id = f.id();
            let message = format!("Field {} is missing", id);
            quote! {
                if self.#id.is_none() {
                    errors.push(#message.into());
                }
            }
        });
        let assign = self.builder.group(self.properties().result_build());
        let create = quote! {
            #target #assign
        };
        let success = match self.builder.mode {
            Mode::Panic => create,
            Mode::Result => quote!(Ok(#create)),
            _ => panic!("Unsupported mode {:?}", self.builder.mode),
        };
        let error = match self.builder.mode {
            Mode::Panic => quote!(panic!("{}", errors.join("\n"))),
            Mode::Result => quote!(Err(errors.join("\n"))),
            _ => panic!("Unsupported mode {:?}", self.builder.mode),
        };
        quote! {
            let mut errors: ::std::vec::Vec<::std::string::String> = ::std::vec![];

            #check_fields

            if !errors.is_empty() {
                #error
            } else {
                #success
            }
        }
    }

    pub fn impl_builder_from(&self) -> TokenStream {
        let builder_name = &self.builder.ident;
        let target = &self.builder.target;
        match self.builder.mode {
            Mode::Panic =>
                quote! {
                    impl ::core::convert::From<#builder_name> for #target {
                        fn from(builder: #builder_name) -> Self {
                            builder.build()
                        }
                    }
                },
            Mode::Result =>
                quote! {
                    impl ::core::convert::TryFrom<#builder_name> for #target {
                        type Error = ::std::string::String;
                        fn try_from(builder: #builder_name) -> ::core::result::Result<Self, Self::Error> {
                            builder.build()
                        }
                    }
                },
            _ => panic!("Unsupported mode {:?}", self.builder.mode),
        }
    }
}

impl Generator for ResultPanicGenerator {
    fn builder(&self) -> &Builder {
        &self.builder
    }

    /// Generate all declarations:
    ///
    /// * `impl` block to add `builder()` function to target struct ([`Self::impl_target()`])
    /// * builder struct ([`Self::struct_builder()`])
    /// * `impl` block for generated builder struct ([`Self::impl_builder()`])
    fn all(&self) -> TokenStream {
        let impl_target = self.impl_target();
        let struct_builder = self.struct_builder();
        let impl_builder = self.impl_builder();
        quote! {
            #impl_target
            #struct_builder
            #impl_builder
        }
    }
}
