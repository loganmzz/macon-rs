//! Generator panicing on missing fields and ignore setting many times.

use super::*;

pub struct ResultPanicGenerator {
    pub builder: Builder,
}

impl ResultPanicGenerator {
    /// Transform all field in a token serie.
    fn with_fields<F: Fn(&Property)->TokenStream>(&self, to_token: F) -> TokenStream {
        self.builder().properties
            .iter()
            .map(to_token)
            .collect()
    }

    /// Generate builder struct
    pub fn struct_builder(&self) -> TokenStream {
        if self.builder.is_tuple {
            self.struct_builder_tuple()
        } else {
            self.struct_builder_named()
        }
    }
    pub fn struct_builder_tuple(&self) -> TokenStream {
        let fields = self.with_fields(|f| {
            let ty = &f.ty;
            quote! {
                Option<#ty>,
            }
        });
        let builder_name = &self.builder.ident;
        quote! {
            #[derive(Default)]
            pub struct #builder_name(
                #fields
            );
        }
    }
    pub fn struct_builder_named(&self) -> TokenStream {
        let fields = self.with_fields(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            quote! {
                pub #ident: Option<#ty>,
            }
        });
        let builder_name = &self.builder.ident;
        quote! {
            #[derive(Default)]
            pub struct #builder_name {
                #fields
            }
        }
    }

    /// Generate `impl` block for generated builder struct:
    ///
    /// * fluent field setters ([impl_builder_setters])
    /// * final `build()` function ([impl_builder_build])
    pub fn impl_builder(&self) -> TokenStream {
        let builder_name = &self.builder.ident;
        let impl_builder_setters = self.impl_builder_setters();
        let impl_builder_build = self.impl_builder_build();
        quote! {
            impl #builder_name {
                #impl_builder_setters
                #impl_builder_build
            }
        }
    }

    /// Generate fluent field setters
    pub fn impl_builder_setters(&self) -> TokenStream {
        self.with_fields(|f| {
            let method = if self.builder.is_tuple {
                let method = format_ident!("set{}", f.ordinal);
                quote!(#method)
            } else {
                f.id()
            };
            let typevar = f.typevar();
            let id = f.id();
            let ty = &f.ty;
            quote! {
                pub fn #method<#typevar: Into<#ty>>(mut self, value: #typevar) -> Self {
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

        let check_fields = self.with_fields(|f| {
            let id = f.id();
            let message = format!("Field {} is missing", id);
            quote! {
                if self.#id.is_none() {
                    errors.push(#message.into());
                }
            }
        });
        let create = if self.builder.is_tuple {
            let assign = self.with_fields(|f| {
                let id = f.id();
                quote!(self.#id.unwrap(),)
            });
            quote! {
                #target(
                    #assign
                )
            }
        } else {
            let assign = self.with_fields(|f| {
                let id = f.id();
                quote!(#id: self.#id.unwrap(),)
            });
            quote! {
                #target {
                    #assign
                }
            }
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
}

impl Generator for ResultPanicGenerator {
    fn builder(&self) -> &Builder {
        &self.builder
    }

    /// Generate all declarations:
    ///
    /// * `impl` block to add `builder()` function to target struct ([impl_target])
    /// * builder struct ([struct_builder])
    /// * `impl` block for generated builder struct ([impl_builder])
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
