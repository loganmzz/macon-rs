use std::{
  fmt::{
    Debug,
    Display,
  },
  ops::Deref,
};
use quote::ToTokens;
use syn::Type;

pub struct TokenDisplay<T: ToTokens>(T);

pub type MType = TokenDisplay<Type>;

impl<T: ToTokens> Debug for TokenDisplay<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      f.write_fmt(format_args!("{}", self))
  }
}

impl<T: ToTokens> Display for TokenDisplay<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("{}", self.0.to_token_stream()))
  }
}

impl<T: ToTokens> From<T> for TokenDisplay<T>  {
  fn from(value: T) -> Self {
      Self(value)
  }
}

impl<T: ToTokens> Deref for TokenDisplay<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: ToTokens> ToTokens for TokenDisplay<T> {
  fn to_token_stream(&self) -> proc_macro2::TokenStream {
      self.0.to_token_stream()
  }

  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
      self.0.to_tokens(tokens)
  }

  fn into_token_stream(self) -> proc_macro2::TokenStream
      where
          Self: Sized, {
      self.0.into_token_stream()
  }
}

impl<T: ToTokens + PartialEq> PartialEq<T> for TokenDisplay<T> {
  fn eq(&self, other: &T) -> bool {
      self.0.eq(other)
  }
}