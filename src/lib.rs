//! Another builder macro-based generator with its own idioms.
//!
//! Usage:
//!
//! ```
//! # #[macro_use] extern crate macon;
//! #[derive(Builder)]
//! struct MyType {
//!   integer: i32,
//!   string: String,
//! }
//!
//! let _mytype: MyType = MyType::builder()
//!     .integer(42)
//!     .string(String::from("foobar"))
//!     .build();
//! ```
//!
//! Features:
//!
//! * adds a builder struct (`<TargetStruct>Builder`)
//! * build struct implements [Default]
//! * adds a `builder()` function to target struct to initialize a new builder
//! * each target struct field can be set with function of same name and parameter of same type
//! * use `build()` function to create new target struct instance
//! * any unset field will make `build()` panic
//!

use proc_macro2::{
    Ident,
    TokenStream,
};
use quote::{
    format_ident,
    quote,
};
use syn::{
    parse_macro_input,
    Data,
    DeriveInput,
    Field,
    Fields,
};

/// Generate all declarations:
///
/// * `impl` block to add `builder()` function to target struct ([impl_target])
/// * builder struct ([struct_builder])
/// * `impl` block for generated builder struct ([impl_builder])
fn all_declarations(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let builder_name = format_ident!("{}Builder", name);

    let fields = match input.data {
        Data::Struct(data) => {
            match data.fields {
                Fields::Named(fields_named) => {
                    fields_named
                        .named
                        .into_iter()
                        .collect::<Vec<_>>()
                },
                Fields::Unit => {
                    vec![]
                },
                _ => {
                    panic!("Only unit or named struct is supported.");
                },
            }
        },
        _ => {
            panic!("Only struct are supported, neither enum, neither union.");
        }
    };

    let impl_target = impl_target(&name, &builder_name);
    let struct_builder = struct_builder(&builder_name, &fields);
    let impl_builder = impl_builder(&name, &builder_name, &fields);
    quote! {
        #impl_target
        #struct_builder
        #impl_builder
    }
}

/// Generate `impl` block to add `builder()` function to target struct
fn impl_target(name: &Ident, builder_name: &Ident) -> TokenStream {
    quote! {
        impl #name {
            pub fn builder() -> #builder_name {
                <#builder_name as core::default::Default>::default()
            }
        }
    }
}

/// Generate builder struct
fn struct_builder(builder_name: &Ident, fields: &Vec<Field>) -> TokenStream {
    let fields = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            quote! {
                pub #ident: Option<#ty>,
            }
        })
        .collect::<TokenStream>();
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
fn impl_builder(name: &Ident, builder_name: &Ident, fields: &Vec<Field>) -> TokenStream {
    let impl_builder_setters = impl_builder_setters(fields);
    let impl_builder_build = impl_builder_build(name, fields);
    quote! {
        impl #builder_name {
            #impl_builder_setters
            #impl_builder_build
        }
    }
}

/// Generate fluent field setters
fn impl_builder_setters(fields: &Vec<Field>) -> TokenStream {
    fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            let ty = &f.ty;
            quote! {
                pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                    self.#ident = Some(#ident);
                    self
                }
            }
        })
        .collect()
}

/// Generate final `build()` function
fn impl_builder_build(name: &Ident, fields: &Vec<Field>) -> TokenStream {
    let body = if fields.is_empty() {
        quote! {
            #name
        }
    } else {
        let build_fields = fields
            .iter()
            .map(|f| {
                let ident = &f.ident;
                quote! {
                    #ident: self.#ident.take().unwrap(),
                }
            })
            .collect::<TokenStream>();
        quote! {
            #name {
                #build_fields
            }
        }
    };
    quote! {
        pub fn build(&mut self) -> #name {
            #body
        }
    }
}

/// Macro entrypoint generating builder:
///
/// * `impl` block to add `builder()` function to target struct
/// * builder struct
/// * `impl` block for generated builder struct
///     * fluent field setters
///     * final `build()` function
#[proc_macro_derive(Builder)]
pub fn builder_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    proc_macro::TokenStream::from(all_declarations(input))
}
