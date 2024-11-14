//! Generator using typestate pattern to avoid invalid usage:
//! * Setting a field many times
//! * Building with unset field

use quote::ToTokens;

use super::*;

pub struct StateGenerator {
    builder: crate::model::Builder,
}

impl StateGenerator {
    pub fn new(builder: Builder) -> Self {
        Self {
            builder,
        }
    }
}

impl Generator for StateGenerator {

    fn builder(&self) -> &Builder {
        &self.builder
    }

    /// Generate all declarations:
    ///
    /// * `impl` block to add `builder()` function to target struct ([`Self::impl_target`])
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

impl StateGenerator {

    pub fn struct_builder(&self) -> TokenStream {
        let vis = &self.builder.vis;
        let builder_name = &self.builder.ident;
        let typestate_default = self.properties().typestate_default();
        let fields = self.properties().typestate_struct_fields();
        let delim = if self.properties().is_tuple {
            quote!(;)
        } else {
            quote!()
        };
        quote! {
            #[derive(Default,)]
            #vis struct #builder_name<#typestate_default>#fields #delim
        }
    }

    pub fn impl_builder(&self) -> TokenStream {
        // let impl_builder_default = self.impl_builder_default();
        let setters = self.properties().to_token(|f| self.impl_builder_setter(f));
        let build = self.impl_builder_build();
        let from_impl = self.impl_builder_from();
        quote! {
            // #impl_builder_default
            #setters
            #build
            #from_impl
        }
    }

    pub fn impl_builder_setter(&self, field: &Property) -> TokenStream {
        let builder_name = &self.builder.ident;
        let impl_state = self.properties().typestate_setter_impl(field);
        let struct_state_from = self.properties().typestate_state(field, false, false);
        let struct_state_to = self.properties().typestate_state(field, false, true);


        let ident = &field.ident;
        let into_type = field.ty_into();
        let argtype = if ! field.into.is_disabled() {
            field.typevar().to_token_stream()
        } else {
            field.ty.to_token_stream()
        };
        let generic = if ! field.into.is_disabled() {
            quote!(<#argtype: ::core::convert::Into<#into_type>>)
        } else {
            quote!()
        };

        let setter_standard = {
            let setter_standard = field.setter();
            let fields_standard = self.properties().typestate_assign(field, Setter::Standard);
            quote! {
                pub fn #setter_standard #generic(self, #ident: #argtype) -> #builder_name<#struct_state_to> {
                    #builder_name #fields_standard
                }
            }
        };

        let setter_option = if field.option.is_enabled() {
            let setter_none = field.setter_none();
            let fields_none = self.properties().typestate_assign(field, Setter::None);
            let setter_optional = field.setter_optional();
            let generic = if ! field.into.is_disabled() {
                quote!(<#argtype: ::core::convert::Into<#into_type>>)
            } else {
                quote!()
            };
            let fields_optional = self.properties().typestate_assign(field, Setter::Optional);
            quote! {
                pub fn #setter_none(self) -> #builder_name<#struct_state_to> {
                    #builder_name #fields_none
                }

                pub fn #setter_optional #generic(self, #ident: ::core::option::Option<#argtype>) -> #builder_name<#struct_state_to> {
                    #builder_name #fields_optional
                }
            }
        } else {
            quote!()
        };
        let setter_keep = if field.struct_default.is_enabled() {
            let setter_keep = field.setter_keep();
            let fields_keep = self.properties().typestate_assign(field, Setter::Keep);
            quote! {
                pub fn #setter_keep(self) -> #builder_name<#struct_state_to> {
                    #builder_name #fields_keep
                }
            }
        } else {
            quote!()
        };
        let setter_default = if field.default.is_enabled() {
            let setter_default = field.setter_default();
            let fields_default = self.properties().typestate_assign(field, Setter::Default);
            quote! {
                pub fn #setter_default(self) -> #builder_name<#struct_state_to> {
                    #builder_name #fields_default
                }
            }
        } else {
            quote!()
        };
        let mut impl_setter = quote! {
            impl<#impl_state> #builder_name<#struct_state_from> {
                #setter_standard
                #setter_option
                #setter_keep
                #setter_default
            }
        };
        if self.builder.is_tuple {
            let struct_state_from_ordered = self.properties().typestate_state(field, true, false);
            let struct_state_to_ordered = self.properties().typestate_state(field, true, true);
            let typevar = field.typevar();
            let into_type = field.ty_into();
            let ident = &field.ident;
            let setter_standard_ordered = {
                let setter_standard = field.setter();
                quote! {
                    pub fn set<#typevar: ::core::convert::Into<#into_type>>(self, #ident: #typevar) -> #builder_name<#struct_state_to_ordered> {
                        self.#setter_standard(#ident)
                    }
                }
            };
            let setter_option_ordered = if field.option.is_enabled() {
                let setter_none = field.setter_none();
                let setter_optional = field.setter_optional();
                quote! {
                    pub fn none(self) -> #builder_name<#struct_state_to_ordered> {
                        self.#setter_none()
                    }
                    pub fn optional<#typevar: ::core::convert::Into<#into_type>>(self, #ident: ::core::option::Option<#typevar>) -> #builder_name<#struct_state_to_ordered> {
                        self.#setter_optional(#ident)
                    }
                }
            } else {
                quote!()
            };
            let setter_keep_ordered = if field.struct_default.is_enabled() {
                let setter_keep = field.setter_keep();
                quote! {
                    pub fn keep(self) -> #builder_name<#struct_state_to_ordered> {
                        self.#setter_keep()
                    }
                }
            } else {
                quote!()
            };
            let setter_default_ordered = if field.default.is_enabled() {
                let setter_default = field.setter_default();
                quote! {
                    pub fn default(self) -> #builder_name<#struct_state_to_ordered> {
                        self.#setter_default()
                    }
                }
            } else {
                quote!()
            };
            impl_setter = quote! {
                #impl_setter

                impl #builder_name<#struct_state_from_ordered> {
                    #setter_standard_ordered
                    #setter_option_ordered
                    #setter_keep_ordered
                    #setter_default_ordered
                }
            }
        }
        impl_setter
    }

    pub fn impl_builder_build(&self) -> TokenStream {
        let option_typevars = self.properties().typestate_optional_marker();
        let builder_name = &self.builder.ident;
        let final_state = self.properties().typestate_state_final();
        let target = &self.builder.target;
        let content = if self.builder.default.is_enabled() {
            self.impl_builder_build_from_default()
        } else {
            self.impl_builder_build_from_scratch()
        };
        quote! {
            impl<#option_typevars> #builder_name<#final_state> {
                pub fn build(self) -> #target {
                    #content
                }
            }
        }
    }

    pub fn impl_builder_build_from_default(&self) -> TokenStream {
        let target = &self.builder.target;
        let overrides = self.properties().typestate_override();
        quote! {
            let mut built = <#target as ::core::default::Default>::default();
            #overrides
            built
        }
    }

    pub fn impl_builder_build_from_scratch(&self) -> TokenStream {
        let target = &self.builder.target;
        let assign = self.properties().typestate_build();
        quote! {
            #target #assign
        }
    }

    pub fn impl_builder_from(&self) -> TokenStream {
        let builder_name = &self.builder.ident;
        let target = &self.builder.target;
        let final_state = self.properties().typestate_state_final();
        let option_typevars: TokenStream = self.properties().typestate_optional_marker();
        quote! {
            impl<#option_typevars> ::core::convert::From<#builder_name<#final_state>> for #target {
                fn from(builder: #builder_name<#final_state>) -> Self {
                    builder.build()
                }
            }
        }
    }
}
