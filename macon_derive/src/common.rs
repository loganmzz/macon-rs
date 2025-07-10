use std::{
    fmt,
};
use proc_macro2::Span;
use serde::{
    de::{
        Visitor,
    },
    Deserialize,
};
use syn::{
  Error,
  Result,
  Token,
  Type,
  parse_str,
};



#[derive(Debug)]
pub struct SpanSetting<T> {
    pub span: Option<Span>,
    pub setting: Setting<T>,
}
impl <T> Default for SpanSetting<T> {
    fn default() -> Self {
        Self {
            span: Default::default(),
            setting: Default::default(),
        }
    }
}
impl<T: Copy> Copy for SpanSetting<T> {}
impl<T: Clone> Clone for SpanSetting<T> {
    fn clone(&self) -> Self {
        SpanSetting { span: self.span.clone(), setting: self.setting.clone(), }
    }
}
impl<T> From<Setting<T>> for SpanSetting<T> {
    fn from(setting: Setting<T>) -> Self {
        SpanSetting { span: None, setting, }
    }
}
impl<T> From<(Span, Setting<T>)> for SpanSetting<T> {
    fn from(value: (Span, Setting<T>)) -> Self {
        let (span, setting) = value;
        SpanSetting { span: Some(span), setting, }
    }
}
impl<T> SpanSetting<T> {
  pub fn as_pair(&self) -> (Span, &Setting<T>) {
    (self.span.as_ref().cloned().unwrap_or_else(Span::call_site), &self.setting)
  }
}
impl<T: PartialEq> PartialEq for SpanSetting<T> {
    fn eq(&self, other: &Self) -> bool {
        self.setting == other.setting
    }
}
impl<T: PartialEq> PartialEq<Setting<T>> for SpanSetting<T> {
    fn eq(&self, other: &Setting<T>) -> bool {
        self.setting == *other
    }
}

#[derive(Debug, Default)]
pub enum Setting<T> {
  #[default]
  Undefined,
  Disabled,
  Enabled(T),
}

impl<T: Copy> Copy for Setting<T> {}
impl<T: Clone> Clone for Setting<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Undefined => Self::Undefined,
            Self::Disabled => Self::Disabled,
            Self::Enabled(value) => Self::Enabled(value.clone()),
        }
    }
}

impl<T: PartialEq> PartialEq for Setting<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Undefined, Self::Undefined) => true,
            (Self::Disabled, Self::Disabled) => true,
            (Self::Enabled(self_value), Self::Enabled(other_value)) => self_value == other_value,
            _ => false,
        }
    }
}

impl<T> Setting<T> {
  pub fn undefined() -> Self {
      Setting::Undefined
  }
  pub fn disable() -> Self {
      Setting::Disabled
  }
  pub fn enable(value: T) -> Self {
      Setting::Enabled(value)
  }

  pub fn is_defined(&self) -> bool {
      !self.is_undefined()
  }

  pub fn is_undefined(&self) -> bool {
      matches!(self, Self::Undefined)
  }

  pub fn is_disabled(&self) -> bool {
      matches!(self, Self::Disabled)
  }

  pub fn is_enabled(&self) -> bool {
      matches!(self, Self::Enabled(_))
  }

  pub fn value(&self) -> Option<&T> {
      match self {
          Self::Enabled(ref value) => Some(value),
          _ => None,
      }
  }

  pub fn map<F,U>(self, f: F) -> Setting<U> where F: FnOnce(T)->U {
      self.and_then(|t| Setting::enable(f(t)))
  }

  pub fn and(self, set: Self) -> Self {
    match (self,set) {
        (Self::Undefined,_) => Self::Undefined,
        (_,Self::Undefined) => Self::Undefined,
        (_,set)             => set,
    }
  }
  pub fn and_then<F,U>(self, f: F) -> Setting<U> where F: FnOnce(T)->Setting<U> {
      match self {
          Self::Undefined => Setting::undefined(),
          Self::Disabled => Setting::disable(),
          Self::Enabled(value) => f(value),
      }
  }

  pub fn or(self, set: Self) -> Self {
    match (self,set) {
        (Self::Undefined, res) => res,
        (res,_) => res,
    }
  }
}

impl Setting<()> {
  pub fn from_parse_nested_meta(nested: syn::meta::ParseNestedMeta) -> Result<(Span, Self)> {
      if nested.input.peek(Token![=]) {
          let value = nested
              .value()
              .map_err_context("Unable to parse setting as value")?;
          match value.parse::<Type>().map_err_context("Unable to parse setting type value")? {
              Type::Never(_) => Ok((value.span(), Self::disable())),
              _ => Err(nested.error(format!("Unsupported setting value {value:?}"))),
          }
      } else {
          Ok((nested.input.span(), Self::enable(())))
      }
  }
}

impl Setting<Type> {
  pub fn from_parse_nested_meta(nested: syn::meta::ParseNestedMeta) -> Result<(Span, Self)> {
      let value = nested
          .value()
          .map_err_context("Unable to parse setting value")?;
      let ty: Type = value
          .parse()
          .map_err_context("Unable to parse setting Type")?;
      Ok((
        value.span(),
        match ty {
          Type::Tuple(ref typetuple) => {
              if typetuple.elems.is_empty() {
                  Setting::enable(ty)
              } else {
                  Setting::disable()
              }
          },
          Type::Never(_) => Setting::disable(),
          _ => Setting::enable(ty),
        },
      ))
  }
}

impl TryFrom<&str> for Setting<Type> {
    type Error = Error;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let str_setting: Setting<String> = value.into();
        match str_setting {
            Setting::Undefined => Ok(Setting::undefined()),
            Setting::Disabled  => Ok(Setting::disable()),
            Setting::Enabled(value) => parse_str::<Type>(&value).map(|ty| Self::enable(ty)),
        }
    }
}
impl From<&str> for Setting<String> {
    fn from(value: &str) -> Self {
        if "!" == value || "false" == value {
            Setting::disable()
        } else {
            Setting::enable(value.to_owned())
        }
    }
}
impl From<bool> for Setting<()> {
    fn from(value: bool) -> Self {
        if value {
            Setting::enable(())
        } else {
            Setting::disable()
        }
    }
}

impl From<Setting<String>> for Setting<()> {
    fn from(setting: Setting<String>) -> Self {
        match setting {
            Setting::Undefined => Setting::undefined(),
            Setting::Disabled => Setting::disable(),
            Setting::Enabled(_) => Setting::enable(()),
        }
    }
}

struct FlagSettingSerdeVisitor;
impl<'de> Visitor<'de> for FlagSettingSerdeVisitor {
    type Value = Setting<()>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a boolean, \"!\" or null")
    }

    fn visit_bool<E>(self, value: bool) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(value.into())
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error, {
        if "!" == value {
            Ok(Setting::disable())
        } else {
            Err(E::custom(format!("invalid string value: {:?}", value)))
        }
    }

    fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(Setting::undefined())
    }
}
impl<'de> Deserialize<'de> for Setting<()> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        deserializer.deserialize_any(FlagSettingSerdeVisitor)
    }
}

#[derive(Default)]
struct StringSettingSerdeVisitor;
impl<'de> Visitor<'de> for StringSettingSerdeVisitor {
    type Value = Setting<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("false as boolean or string, \"!\" or null")
    }

    fn visit_bool<E>(self, value: bool) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(if !value {
            Setting::disable()
        } else {
            "true".into()
        })
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(value.into())
    }

    fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(Setting::undefined())
    }
}
impl<'de> Deserialize<'de> for Setting<String> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        deserializer.deserialize_any(StringSettingSerdeVisitor)
    }
}

#[derive(Default)]
struct TypeSettingSerdeVisitor(StringSettingSerdeVisitor);
impl<'de> Visitor<'de> for TypeSettingSerdeVisitor {
    type Value = Setting<Type>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("false as boolean or string, \"!\" or null")
    }

    fn visit_bool<E>(self, value: bool) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error, {
        if !value {
            Ok(Setting::disable())
        } else {
            "true"
                .try_into()
                .map_err(|err|
                    E::custom(err)
                )
        }
    }

    fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error, {
        value
                .try_into()
                .map_err(|err| E::custom(err))
    }

    fn visit_none<E>(self) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error, {
        Ok(Setting::undefined())
    }
}
impl<'de> Deserialize<'de> for Setting<Type> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        deserializer.deserialize_any(TypeSettingSerdeVisitor::default())
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
