//! Macro implementation crate generating builders for [macon](https://crates.io/crates/macon/0.3.0).
//!
//! See it for all details.
//!


use syn::{
    parse_macro_input,
    DeriveInput,
};

mod attributes;
mod common;
mod config;
mod model;
mod generators;

/// Derive macro to generate builder for your structs. See crate documentation for usage.
///
/// ```compile_fail
/// # #[macro_use] extern crate macon;
/// #[derive(Builder)]
/// #[derive(Debug,PartialEq,)]
/// struct MyType {
///   integer: i32,
///   string: String,
///   optional: Option<String>,
/// }
///
/// let built = MyType::builder()
///     .integer(42)
///     .string("foobar")
///     .build();
///
/// assert_eq!(
///   MyType {
///     integer: 42,
///     string: String::from("foobar"),
///     optional: None,
///   },
///   built,
/// );
/// ```
#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let builder = model::Builder::from_input(input).unwrap();
    let generator: Box<dyn generators::Generator> = builder.into();
    let value = generator.all();
    // eprintln!("{:#?}", value);
    proc_macro::TokenStream::from(value)
}
