//! Generator using typestate pattern to avoid invalid usage:
//! * Setting a field many times
//! * Building with unset field

use super::*;

pub struct StateGenerator {
    builder: crate::model::Builder,
}

struct OutputProperty<'a>(&'a Property);
struct OutputProperties<'a>(Vec<OutputProperty<'a>>);

impl<'a> From<&'a Property> for OutputProperty<'a> {
    fn from(value: &'a Property) -> Self {
        Self(value)
    }
}
impl<'a> From<&'a Builder> for OutputProperties<'a> {
    fn from(value: &'a Builder) -> Self {
        Self(value.properties.iter().map(|p| p.into()).collect())
    }
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
        let init_fields = self.properties().typestate_init();
        let optional_typestate = self.properties().typestate_optional_marker();
        let marker_field = if self.builder.is_tuple {
            quote!(::core::marker::PhantomData<(#optional_typestate)>,)
        } else {
            quote!(__optional_set: ::core::marker::PhantomData<(#optional_typestate)>,)
        };
        let group = self.builder.group(quote! {
            #init_fields
            #marker_field
        });
        let delim = if self.builder.is_tuple {
            quote!(;)
        } else {
            quote!()
        };
        quote! {
            #[derive(Default)]
            #vis struct #builder_name<#typestate_default>#group #delim
        }
    }

    pub fn impl_builder(&self) -> TokenStream {
        let setters = self.properties().to_token(|f| self.impl_builder_setter(f));
        let build = self.impl_builder_build();
        let from_impl = self.impl_builder_from();
        quote! {
            #setters
            #build
            #from_impl
        }
    }

    pub fn impl_builder_setter(&self, field: &Property) -> TokenStream {
        let builder_name = &self.builder.ident;
        let ident = &field.ident;
        let into_type = field.option.value().unwrap_or(&field.ty);
        let setter = field.setter();
        let impl_state = self.properties().typestate_setter_impl(field);
        let typevar = field.typevar();
        let struct_state_from = self.properties().typestate_state(field, false, false);
        let struct_state_to = self.properties().typestate_state(field, false, true);
        let values = self.properties().assign(field, false, false);
        let marker_field = if self.builder.is_tuple {
            quote!(::core::default::Default::default(),)
        } else {
            quote!(__optional_set: ::core::default::Default::default(),)
        };
        let all_values = self.builder.group(quote! {
            #values
            #marker_field
        });
        let setter_none = if field.option.is_enabled() {
            let setter_none = field.setter_none();
            let assign_none = self.properties().assign(field, true, false);
            let assign_none_all = self.builder.group(quote! {
                #assign_none
                #marker_field
            });
            quote! {
                pub fn #setter_none(self) -> #builder_name<#struct_state_to> {
                    #builder_name #assign_none_all
                }
            }
        } else {
            quote!()
        };
        let setter_keep = if field.default.is_enabled() {
            let setter_keep = field.setter_keep();
            let assign_keep = self.properties().assign(field, false, true);
            let assign_keep_all = self.builder.group(quote! {
                #assign_keep
                #marker_field
            });
            quote! {
                pub fn #setter_keep(self) -> #builder_name<#struct_state_to> {
                    #builder_name #assign_keep_all
                }
            }
        } else {
            quote!()
        };
        let mut impl_setter = quote! {
            impl<#impl_state> #builder_name<#struct_state_from> {
                pub fn #setter<#typevar: ::core::convert::Into<#into_type>>(self, #ident: #typevar) -> #builder_name<#struct_state_to> {
                    #builder_name #all_values
                }
                #setter_none
                #setter_keep
            }
        };
        if self.builder.is_tuple {
            let struct_state_from_ordered = self.properties().typestate_state(field, true, false);
            let struct_state_to_ordered = self.properties().typestate_state(field, true, true);
            let setter_none_ordered = if field.option.is_enabled() {
                let setter_none = field.setter_none();
                quote! {
                    pub fn none(self) -> #builder_name<#struct_state_to_ordered> {
                        self.#setter_none()
                    }
                }
            } else {
                quote!()
            };
            let setter_keep_ordered = if field.default.is_enabled() {
                let setter_keep = field.setter_keep();
                quote! {
                    pub fn keep(self) -> #builder_name<#struct_state_to_ordered> {
                        self.#setter_keep()
                    }
                }
            } else {
                quote!()
            };
            impl_setter = quote! {
                #impl_setter

                impl #builder_name<#struct_state_from_ordered> {
                    pub fn set<#typevar: ::core::convert::Into<#into_type>>(self, #ident: #typevar) -> #builder_name<#struct_state_to_ordered> {
                        self.#setter(#ident)
                    }
                    #setter_none_ordered
                    #setter_keep_ordered
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
        let overrides = self.properties().typestate_build_overrides();
        quote! {
            let mut built = <#target as ::core::default::Default>::default();
            #overrides
            built
        }
    }

    pub fn impl_builder_build_from_scratch(&self) -> TokenStream {
        let target = &self.builder.target;
        let assign = self.builder.group(self.properties().typestate_build());
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
