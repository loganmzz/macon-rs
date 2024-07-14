//! API crate consumed by generated builders from [macon](https://crates.io/crates/macon/0.3.0).
//!
//! See it for all details.
//!

use std::{
    fmt::Debug, ops::Deref, vec
};

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

/// Builder field type when target implment [`Extend`].
pub struct Extending<C,I> {
    /// Collecting items
    items: Vec<I>,
    /// Building value
    value: C,
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

impl<C, I> Default for Extending<C, I> where C: Default {
    fn default() -> Self {
        Self {
            items: Default::default(),
            value: Default::default(),
        }
    }
}

impl<C, I> Deref for Extending<C, I> {
    type Target = C;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<C, I> Extend<I> for Extending<C, I> {
    /// Store `iter` values into `items` (until container is created)
    fn extend<T: IntoIterator<Item = I>>(&mut self, iter: T) {
        self.items.extend(iter)
    }
}

impl<C, I> IntoIterator for Extending<C, I> {
    type Item = I;
    type IntoIter = vec::IntoIter<I>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<C, I> Extending<C, I>  {
    pub fn with_value(self, value: C) -> Self {
        Self {
            value,
            items: self.items,
        }
    }

    pub fn value_mut(&mut self) -> &mut C {
        &mut self.value
    }

    /// Consume to return `value` and collected `items`.
    pub fn unwrap(self) -> (C, Vec<I>) {
        (self.value, self.items)
    }

    /// Consume to return built collection extracting `value` and extending with ìtems.
    pub fn unwrap_with<IS: Extend<I>, F: FnOnce(C)->IS>(self, f: F) -> IS {
        let Self { value, items } = self;
        let mut unwrapped = f(value);
        unwrapped.extend(items);
        unwrapped
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
