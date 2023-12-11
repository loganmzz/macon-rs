//! Generator panicing on missing fields and ignore setting many times.

use super::*;

pub struct PanicGenerator {
    pub builder: Builder,
}

impl PanicGenerator {
    /// Transform all field in a token serie.
    fn with_fields<F: Fn(&Property)->TokenStream>(&self, to_token: F) -> TokenStream {
        self.builder().properties
            .iter()
            .map(to_token)
            .collect()
    }

    /// Generate builder struct
    pub fn struct_builder(&self) -> TokenStream {
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
            let ident = &f.ident;
            let ty = &f.ty;
            quote! {
                pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            }
        })
    }

    /// Generate final `build()` function
    pub fn impl_builder_build(&self) -> TokenStream {
        let name = &self.builder.target;
        let body = if self.builder.properties.is_empty() {
            quote! {
                #name
            }
        } else {
            let build_fields = self.with_fields(|f| {
                let ident = &f.ident;
                quote! {
                    #ident: self.#ident.take().unwrap(),
                }
            });
            let check_fields = self.with_fields(|f| {
                let ident = &f.ident;
                let message = format!("Field {} is missing", ident);
                quote! {
                    if self.#ident.is_none() {
                        errors.push(String::from(#message));
                    }
                }
            });
            quote! {
                let mut errors: Vec<String> = vec![];

                #check_fields

                if ! errors.is_empty() {
                    panic!("{}", errors.join("\n"))
                } else {
                    #name {
                        #build_fields
                    }
                }
            }
        };
        quote! {
            pub fn build(&mut self) -> #name {
                #body
            }
        }
    }


}

impl Generator for PanicGenerator {
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
