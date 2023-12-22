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
        if self.builder.is_tuple {
            self.struct_builder_tuple()
        } else {
            self.struct_builder_named()
        }
    }

    pub fn struct_builder_tuple(&self) -> TokenStream {
        let builder_name = &self.builder.ident;
        let typestate_default = self.with_fields(|f| {
            let typestate = self.field_typestate(f);
            quote!(#typestate=(),)
        });
        let fields: TokenStream = self.with_fields(|f| {
            let typestate = self.field_typestate(f);
            quote! {
                #typestate,
            }
        });
        quote! {
            pub struct #builder_name<#typestate_default>(#fields);
        }
    }

    pub fn struct_builder_named(&self) -> TokenStream {
        let builder_name = &self.builder.ident;
        let typestate_default = self.with_fields(|f| {
            let typestate = self.field_typestate(f);
            quote!(#typestate=(),)
        });
        let fields = self.with_fields(|f| {
            let ident = &f.ident;
            let typestate = self.field_typestate(f);
            quote! {
                #ident: #typestate,
            }
        });
        quote! {
            pub struct #builder_name<#typestate_default> {
                #fields
            }
        }
    }

    pub fn default_builder(&self) -> TokenStream {
        if self.builder.is_tuple {
            self.default_builder_tuple()
        } else {
            self.default_builder_named()
        }
    }

    pub fn default_builder_tuple(&self) -> TokenStream {
        let builder_name = &self.builder.ident;
        let builder_fields_init = self.with_fields(|_| {
            quote! {
                core::default::Default::default(),
            }
        });
        quote! {
            impl core::default::Default for #builder_name {
                fn default() -> Self {
                    Self(
                        #builder_fields_init
                    )
                }
            }
        }

    }

    pub fn default_builder_named(&self) -> TokenStream {
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
                    Self{
                        #builder_fields_init
                    }
                }
            }
        }
    }

    pub fn impl_builder(&self) -> TokenStream {
        let setters = self.with_fields(|f| self.impl_builder_setter(f));
        let build = self.impl_builder_build();
        let from_impl = self.impl_builder_from();
        quote! {
            #setters
            #build
            #from_impl
        }
    }

    pub fn impl_builder_setter(&self, field: &Property) -> TokenStream {
        if self.builder.is_tuple {
            self.impl_builder_setter_tuple(field)
        } else {
            self.impl_builder_setter_named(field)
        }
    }

    pub fn impl_builder_setter_tuple(&self, field: &Property) -> TokenStream {
        let builder_name = &self.builder.ident;
        let typestate = self.field_typestate(field);
        let struct_state_from_ordered = self.with_fields(|f| {
            if f.ordinal < field.ordinal {
                let ty = &f.ty;
                quote!(#ty,)
            } else {
                quote!((),)
            }
        });
        let ident = &field.ident;
        let ty = &field.ty;
        let struct_state_to_ordered = self.with_fields(|f| {
            if f.ordinal <= field.ordinal {
                let ty = &f.ty;
                quote!(#ty,)
            } else {
                quote!((),)
            }
        });
        let impl_state = self.with_fields(|f| {
            if f.name == field.name {
                quote!()
            } else {
                let typestate = self.field_typestate(f);
                quote!(#typestate,)
            }
        });
        let struct_state_from_unordered = self.with_fields(|f| {
            if f.name == field.name {
                quote!((),)
            } else {
                let typestate = self.field_typestate(f);
                quote!(#typestate,)
            }
        });
        let setter = format_ident!("set{}", field.ordinal);
        let struct_state_to_unordered = self.with_fields(|f| {
            if f.name == field.name {
                let ty = &f.ty;
                quote!(#ty,)
            } else {
                let typestate = self.field_typestate(f);
                quote!(#typestate,)
            }
        });
        let values = self.with_fields(|f| {
            if f.name == field.name {
                let ident = &f.ident;
                quote!(#ident.into(),)
            } else {
                let id = f.id();
                quote!(self.#id,)
            }
        });
        quote! {
            impl #builder_name<#struct_state_from_ordered> {
                pub fn set<#typestate: Into<#ty>>(self, #ident: #typestate) -> #builder_name<#struct_state_to_ordered> {
                    self.#setter(#ident)
                }
            }
            impl<#impl_state> #builder_name<#struct_state_from_unordered> {
                pub fn #setter<#typestate: Into<#ty>>(self, #ident: #typestate) -> #builder_name<#struct_state_to_unordered> {
                    #builder_name(#values)
                }
            }
        }

    }

    pub fn impl_builder_setter_named(&self, field: &Property) -> TokenStream {
        let impl_state = self.with_fields(|f| {
            if f.name == field.name {
                quote!()
            } else {
                let typestate = self.field_typestate(f);
                quote!(#typestate,)
            }
        });
        let builder_name = &self.builder.ident;
        let struct_state_from = self.with_fields(|f| {
            if f.name == field.name {
                quote!((),)
            } else {
                let typestate = self.field_typestate(f);
                quote!(#typestate,)
            }
        });
        let ident = &field.ident;
        let typestate = self.field_typestate(field);
        let ty = &field.ty;
        let struct_state_to = self.with_fields(|f| {
            if f.name == field.name {
                let ty = &f.ty;
                quote!(#ty,)
            } else {
                let typestate = self.field_typestate(f);
                quote!(#typestate,)
            }
        });
        let values = self.with_fields(|f| {
            let ident = &f.ident;
            if f.name == field.name {
                quote!(#ident: #ident.into(),)
            } else {
                quote!(#ident: self.#ident,)
            }
        });
        quote! {
            impl<#impl_state> #builder_name<#struct_state_from> {
                pub fn #ident<#typestate: Into<#ty>>(self, #ident: #typestate) -> #builder_name<#struct_state_to> {
                    #builder_name {
                        #values
                    }
                }
            }
        }
    }

    pub fn impl_builder_build(&self) -> TokenStream {
        if self.builder.is_tuple {
            self.impl_builder_build_tuple()
        } else {
            self.impl_builder_build_named()
        }
    }

    pub fn impl_builder_build_tuple(&self) -> TokenStream {
        let builder_name = &self.builder.ident;
        let target = &self.builder.target;
        let final_state = self.with_fields(|f| {
            let ty = &f.ty;
            quote!(#ty,)
        });
        let assign = self.with_fields(|f| {
            let id = f.id();
            quote! {
                self.#id,
            }
        });
        quote! {
            impl #builder_name<#final_state> {
                pub fn build(self) -> #target {
                    #target(
                        #assign
                    )
                }
            }
        }
    }

    pub fn impl_builder_build_named(&self) -> TokenStream {
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

    pub fn impl_builder_from(&self) -> TokenStream {
        let builder_name = &self.builder.ident;
        let target = &self.builder.target;
        let final_state = self.with_fields(|f| {
            let ty = &f.ty;
            quote!(#ty,)
        });
        quote! {
            impl From<#builder_name<#final_state>> for #target {
                fn from(builder: #builder_name<#final_state>) -> Self {
                    builder.build()
                }
            }
        }
    }

    pub fn field_typestate(&self, field: &Property) -> Ident {
        field.typevar()
    }
}
