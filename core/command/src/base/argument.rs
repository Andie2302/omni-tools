use crate::{AffixPair, KeyValuePair};
use core::fmt::{self, Display, Formatter};



#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Formatting<'a> {
    pub affix: AffixPair<'a>,
    pub separator: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Argument<'a> {
    pub key_value_pair: KeyValuePair<'a>,
    pub formatting: Formatting<'a>,
}

impl Display for Argument<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatting.affix.prefix)?;
        write!(f, "{}", self.key_value_pair.key)?;
        // Separator & Value nur rendern, wenn der Value auch tatsächlich Text hat
        if !self.key_value_pair.value.is_empty() {
            write!(f, "{}", self.formatting.separator)?;
            write!(f, "{}", self.key_value_pair.value)?;
        }
        write!(f, "{}", self.formatting.affix.postfix)
    }
}
