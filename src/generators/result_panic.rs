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
            quote! {
                pub fn #method<#typevar: ::core::convert::Into<#ty>>(mut self, value: #typevar) -> Self {
                    self.#id = value.into().into();
                    self
                }
            }
        })
    }

    /// Generate final `build()` function
    pub fn impl_builder_build(&self) -> TokenStream {
        let target = &self.builder.target;
        let output = match self.builder.mode {
            Mode::Panic => quote!(#target),
            Mode::Result => quote!(Result<#target, String>),
            _ => panic!("Unsupported mode {:?}", self.builder.mode),
        };

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
            pub fn build(self) -> #output {
                let mut errors: Vec<String> = vec![];

                #check_fields

                if !errors.is_empty() {
                    #error
                } else {
                    #success
                }
            }
        }
    }

    pub fn impl_builder_from(&self) -> TokenStream {
        let builder_name = &self.builder.ident;
        let target = &self.builder.target;
        match self.builder.mode {
            Mode::Panic =>
                quote! {
                    impl From<#builder_name> for #target {
                        fn from(builder: #builder_name) -> Self {
                            builder.build()
                        }
                    }
                },
            Mode::Result =>
                quote! {
                    impl TryFrom<#builder_name> for #target {
                        type Error = String;
                        fn try_from(builder: #builder_name) -> Result<Self, Self::Error> {
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
