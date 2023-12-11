//! Generator using typestate pattern to avoid invalid usage:
//! * Setting a field many times
//! * Building with unset field

use super::*;

pub struct StateGenerator {
    pub builder: crate::model::Builder,
}

impl Generator for StateGenerator {
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
        let default_builder = self.default_builder();
        let impl_builder = self.impl_builder();
        quote! {
            #impl_target
            #struct_builder
            #default_builder
            #impl_builder
        }
    }
}

impl StateGenerator {
    /// Transform all field in a token serie.
    fn with_fields<F: Fn(&Property)->TokenStream>(&self, to_token: F) -> TokenStream {
        self.builder().properties
            .iter()
            .map(to_token)
            .collect()
    }

    pub fn struct_builder(&self) -> TokenStream {
        let fields = self.with_fields(|f| {
            let ident = &f.ident;
            let typestate = self.field_typestate(f);
            quote! {
                #ident: #typestate,
            }
        });
        let markerstate = self.with_fields(|f| {
            let typestate = self.field_typestate(f);
            quote!(#typestate,)
        });
        let builder_name = &self.builder.ident;
        let typestate_default = self.with_fields(|f| {
            let ident = self.field_typestate(f);
            quote!(#ident=(),)
        });
        quote! {
            pub struct #builder_name<#typestate_default> {
                __marker: core::marker::PhantomData<(#markerstate)>,
                #fields
            }
        }
    }

    pub fn default_builder(&self) -> TokenStream {
        let builder_name = &self.builder.ident;
        let builder_fields_init = self.with_fields(|f| {
            let ident = &f.ident;
            quote! {
                #ident: core::default::Default::default(),
            }
        });
        quote! {
            impl core::default::Default for #builder_name {
                fn default() -> Self {
                    #builder_name {
                        __marker: core::marker::PhantomData,
                        #builder_fields_init
                    }
                }
            }
        }
    }

    pub fn impl_builder(&self) -> TokenStream {
        let setters = self.with_fields(|f| self.impl_builder_setters(f));
        let build = self.impl_builder_build();
        quote! {
            #setters
            #build
        }
    }

    pub fn impl_builder_setters(&self, field: &Property) -> TokenStream {
        let impl_state = self.with_fields(|f| {
            if f.name == field.name {
                quote!()
            } else {
                let typestate = self.field_typestate(f);
                quote!(#typestate,)
            }
        });
        let struct_state_from = self.with_fields(|f| {
            if f.name == field.name {
                quote!((),)
            } else {
                let typestate = self.field_typestate(f);
                quote!(#typestate,)
            }
        });
        let struct_state_to = self.with_fields(|f| {
            if f.name == field.name {
                let ty = &f.ty;
                quote!(#ty,)
            } else {
                let typestate = self.field_typestate(f);
                quote!(#typestate,)
            }
        });
        let assign = self.with_fields(|f| {
            let ident = &f.ident;
            if f.name == field.name {
                quote!(#ident,)
            } else {
                quote!(#ident: self.#ident,)
            }
        });
        let builder_name = &self.builder.ident;
        let ident = &field.ident;
        let ty = &field.ty;
        quote! {
            impl<#impl_state> #builder_name<#struct_state_from> {
                pub fn #ident(self, #ident: #ty) -> #builder_name<#struct_state_to> {
                    #builder_name {
                        __marker: core::marker::PhantomData,
                        #assign
                    }
                }
            }
        }
    }

    pub fn impl_builder_build(&self) -> TokenStream {
        let builder_name = &self.builder.ident;
        let target = &self.builder.target;
        let final_state = self.with_fields(|f| {
            let ty = &f.ty;
            quote!(#ty,)
        });
        let assign = self.with_fields(|f| {
            let ident = &f.ident;
            quote!(#ident: self.#ident,)
        });
        quote! {
            impl #builder_name<#final_state> {
                pub fn build(self) -> #target {
                    #target {
                        #assign
                    }
                }
            }
        }
    }

    pub fn field_typestate(&self, field: &Property) -> Ident {
        format_ident!("{}", field.name.to_uppercase())
    }
}
