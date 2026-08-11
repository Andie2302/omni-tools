use crate::AffixedString;
use core::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KeyValuePair<'a> {
    pub key: Key<'a>,
    pub value: Value<'a>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Key<'a>(pub AffixedString<'a>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Value<'a>(pub AffixedString<'a>);

impl Value<'_> {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for Key<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Value<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}