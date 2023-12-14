pub use crate::model::{
    Builder,
    Mode,
    Property,
};
pub use proc_macro2::{
    Ident,
    TokenStream,
};
pub use quote::{
    format_ident,
    quote,
};

pub mod typestate;
pub mod result_panic;

pub trait Generator {
    fn builder(&self) -> &crate::model::Builder;


    /// Generate all declarations (content vary on strategy)
    fn all(&self) -> TokenStream;

    /// Generate `impl` block to add `builder()` function to target struct
    fn impl_target(&self) -> TokenStream {
        let target = &self.builder().target;
        let builder_name = &self.builder().ident;
        quote! {
            impl #target {
                pub fn builder() -> #builder_name {
                    <#builder_name as core::default::Default>::default()
                }
            }
        }
    }

}

impl From<Builder> for Box<dyn Generator> {
    fn from(builder: Builder) -> Self {
        match builder.mode {
            Mode::Panic => Box::from(result_panic::ResultPanicGenerator { builder }),
            Mode::Typestate => Box::from(typestate::StateGenerator { builder }),
            Mode::Result => Box::from(result_panic::ResultPanicGenerator { builder }),
        }
    }
}
