//! Another builder macro-based generator with its own idioms.
//!
//! "[Maçon](https://fr.wiktionary.org/wiki/ma%C3%A7on#Nom_commun_2)" is French translation for "[builder](https://www.wordreference.com/enfr/builder)"
//!
//! ### Usage
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
//! * adds a builder struct (`<TargetStruct>Builder`)
//! * build struct implements [Default]
//! * adds a `builder()` function to target struct to initialize a new builder
//! * each target struct field can be set with function of same name and parameter of same type
//! * use `build()` function to create new target struct instance
//! * any unset field will make `build()` call not compile (default)
//!
//! ### Features
//!
//! #### Typestate pattern (default)
//!
//! By default, builder rely on typestate pattern. It means state is encoded in type (using generics). Applicable functions are implemented
//! (callable) only when state (type) matches:
//!
//! * Build function `build()` when all properties has been set
//! * Each property setter function as long as property haven't been set
//!
//! Optionally, you can set it explictly:
//!
//! ```
//! # #[macro_use] extern crate macon;
//! #[derive(Builder)]
//! #[builder(mode=Typestate)]
//! struct MyType {
//!   integer: i32,
//!   string: String,
//! }
//!
//! ```
//!
//! #### Panic on `build()`
//!
//! By default, builder rely on typestate pattern to avoid misconfiguration by adding compilation constraint. You can switch to a builder
//! that just panic when misconfigured:
//!
//! ```should_panic
//! # #[macro_use] extern crate macon;
//! #[derive(Builder)]
//! #[builder(mode=Panic)]
//! struct MyType {
//!   integer: i32,
//!   string: String,
//! }
//!
//! let _mytype: MyType = MyType::builder()
//!     .integer(42)
//!     .build();
//! ```
//!
//! #### Result on `build()`
//!
//! By default, builder rely on typestate pattern to avoid misconfiguration by adding compilation constraint. You can switch to a builder
//! that returns a [`Result`]:
//!
//! ```
//! # #[macro_use] extern crate macon;
//! #[derive(Builder)]
//! #[builder(mode=Result)]
//! struct MyType {
//!   integer: i32,
//!   string: String,
//! }
//!
//! let myTypeResult: Result<MyType,String> = MyType::builder()
//!     .integer(42)
//!     .build();
//!
//! assert!(myTypeResult.is_err());
//! ```
//!

use syn::{
    parse_macro_input,
    DeriveInput,
};

mod model;
mod generators;

/// Macro entrypoint generating builder:
///
/// * `impl` block to add `builder()` function to target struct
/// * builder struct
/// * `impl` block(s) for generated builder struct
///     * fluent field setters
///     * final `build()` function
#[proc_macro_derive(Builder, attributes(builder))]
pub fn builder_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let builder = model::Builder::from_input(input).unwrap();
    let generator: Box<dyn generators::Generator> = builder.into();
    proc_macro::TokenStream::from(generator.all())
}
