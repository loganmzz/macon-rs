//! API crate consumed by generated builders from [macon](https://crates.io/crates/macon).
//!
//! See it for all details.
//!

use std::fmt::Debug;

/// Builder field type when building struct implementing [`Default`].
#[derive(Default,)]
pub enum Keeping<T> {
    /// Builder field value when keeping data from [`Default`] instance.
    #[default]
    Keep,
    /// Builder field value to override [`Default`] instance.
    Set(T),
}

/// Buidler field type when target field implement [`Default`].
#[derive(Default,)]
pub enum Defaulting<T: Default> {
    /// Builder field value when using [`Default`] one.
    #[default]
    Default,
    /// Builder field value when using provided data.
    Set(T),
}

/// Builder field type for `Panic` or `Result` mode.
#[derive(Default,)]
pub enum Building<T> {
    /// Builder field value when not set.
    #[default]
    Undefined,
    /// Builder field value when using provided data.
    Set(T),
}

impl<T> Debug for Keeping<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keep => write!(f, "Keeping::Keep"),
            Self::Set(_) => write!(f, "Keeping::Set(_)"),
        }
    }
}

impl<T> Keeping<T> {
    /// Check if [set](Keeping::Set).
    pub fn is_set(&self) -> bool {
        match self {
            Self::Keep => false,
            Self::Set(_) => true,
        }
    }
    /// Consume to return [set](Keeping::Set) value.
    ///
    /// It shouldn't be called for [Keeping::Keep].
    pub fn unwrap(self) -> T {
        match self {
            Self::Keep => panic!("Can't unwrap value from {:#?}", self),
            Self::Set(value) => value,
        }
    }
}

impl<T: Default> Debug for Defaulting<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Default => write!(f, "Defaulting::Default"),
            Self::Set(_) => write!(f, "Defaulting::Set(_)"),
        }
    }
}

impl<T: Default> Defaulting<T> {
    /// Consume to return [default](Defaulting::Default) or [set](Defaulting::Set) value.
    pub fn unwrap(self) -> T {
        match self {
            Self::Default => T::default(),
            Self::Set(value) => value,
        }
    }
}

impl<T> Debug for Building<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undefined => write!(f, "Building::Undefined"),
            Self::Set(_) => write!(f, "Building::Set(_)"),
        }
    }
}

impl<T> Building<T> {
    /// Check if [`Building::Undefined`].
    pub fn is_undefined(&self) -> bool {
        match self {
            Self::Undefined => true,
            _ => false,
        }
    }
    /// Consume to return [set](Defaulting::Set) value.
    ///
    /// It shouldn't be called for [`Building::Undefined`].
    pub fn unwrap(self) -> T {
        match self {
            Self::Set(value) => value,
            _ => panic!("Can't unwrap value from {:#?}", self),
        }
    }
}
