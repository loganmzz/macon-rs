use proc_macro2::Span;
use syn::{
  Error,
  Result,
  Token,
  Type,
};

#[derive(Debug, Default)]
pub enum Setting<T> {
  #[default]
  Undefined,
  Disabled {
    span: Span,
  },
  Enabled {
    value: T,
    span: Span,
  },
}

impl<T: Copy> Copy for Setting<T> {}

impl<T: PartialEq> PartialEq for Setting<T> {
fn eq(&self, other: &Self) -> bool {
    match (self, other) {
        (Self::Undefined, Self::Undefined) => true,
        (Self::Disabled { .. }, Self::Disabled { .. }) => true,
        (Self::Enabled { value: self_value, .. }, Self::Enabled { value: other_value, .. }) => self_value == other_value,
        _ => false,
    }
}
}
impl<T: Clone> Clone for Setting<T> {
fn clone(&self) -> Self {
    match self {
        Self::Undefined => Self::Undefined,
        Self::Disabled { span } => Self::Disabled { span: span.clone() },
        Self::Enabled { value, span }=> Self::Enabled { value: value.clone(), span: span.clone() },
    }
}
}

impl<T> Setting<T> {
  pub fn undefined() -> Self {
      Setting::Undefined
  }
  pub fn disable(span: Span) -> Self {
      Setting::Disabled {
        span,
      }
  }
  pub fn enable(value: T, span: Span) -> Self {
      Setting::Enabled {
        value,
        span,
      }
  }

  pub fn is_defined(&self) -> bool {
      !self.is_undefined()
  }

  pub fn is_undefined(&self) -> bool {
      matches!(self, Self::Undefined)
  }

  pub fn is_disabled(&self) -> bool {
      matches!(self, Self::Disabled { .. })
  }

  pub fn is_enabled(&self) -> bool {
      matches!(self, Self::Enabled { .. })
  }

  pub fn value(&self) -> Option<&T> {
      match self {
          Self::Enabled { ref value, .. } => Some(value),
          _ => None,
      }
  }
  pub fn span(&self) -> Option<&Span> {
    match self {
      Self::Undefined => None,
      Self::Disabled { span } => Some(span),
      Self::Enabled { span , .. } => Some(span),
    }
  }

  pub fn map<F,U>(self, f: F) -> Setting<U> where F: FnOnce(T)->U {
      self.and_then(|t, span| Setting::enable(f(t), span))
  }
  pub fn and_then<F,U>(self, f: F) -> Setting<U> where F: FnOnce(T, Span)->Setting<U> {
      match self {
          Self::Undefined => Setting::undefined(),
          Self::Disabled { span } => Setting::disable(span),
          Self::Enabled { value, span } => f(value, span),
      }
  }
}

impl Setting<()> {
  pub fn from_parse_nested_meta(nested: syn::meta::ParseNestedMeta) -> Result<Self> {
      if nested.input.peek(Token![=]) {
          let value = nested
              .value()
              .map_err_context("Unable to parse setting as value")?;
          match value.parse::<Type>().map_err_context("Unable to parse setting type value")? {
              Type::Never(_) => Ok(Self::disable(value.span())),
              _ => Err(nested.error(format!("Unsupported setting value {value:?}"))),
          }
      } else {
          Ok(Self::enable((), nested.input.span()))
      }
  }
}

impl Setting<Type> {
  pub fn from_parse_nested_meta(nested: syn::meta::ParseNestedMeta) -> Result<Self> {
      let value = nested
          .value()
          .map_err_context("Unable to parse setting value")?;
      let ty: Type = value
          .parse()
          .map_err_context("Unable to parse setting Type")?;
      Ok(match ty {
          Type::Tuple(ref typetuple) => {
              if typetuple.elems.is_empty() {
                  Setting::enable(ty, value.span())
              } else {
                  Setting::disable(value.span())
              }
          },
          Type::Never(_) => Setting::disable(value.span()),
          _ => Setting::enable(ty, value.span()),
      })
  }
}

impl<T> From<(T, Span)> for Setting<T> {
  fn from((value, span): (T, Span)) -> Self {
      Self::enable(value, span)
  }
}

pub trait ResultErrorContext {
  fn map_err_context<C: ::core::fmt::Display>(self, context: C) -> Self;
}
impl<T> ResultErrorContext for Result<T> {
  fn map_err_context<C: ::core::fmt::Display>(self, context: C) -> Self {
      if let Err(err) = self {
          let mut newerr: Option<Error> = None;
          for e in err {
              let contextualized = Error::new(
                  e.span(),
                  format!("{}: {}", context, e),
              );
              if let Some(ref mut error) = newerr {
                  error.extend(vec![contextualized]);
              } else {
                  newerr = Some(contextualized);
              }
          }
          Err(newerr.unwrap())
      } else {
          self
      }
  }
}