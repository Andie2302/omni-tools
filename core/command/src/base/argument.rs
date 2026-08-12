use crate::{AffixPair, KeyValuePair};
use core::fmt::{self, Display, Formatter};



#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Formatting<'a> {
    pub affix: AffixPair<'a>,
    /// Trenner zwischen Key und Value beim Rendern eines `Argument`.
    ///
    /// ACHTUNG: `Formatting::default()` liefert `separator = ""`.
    /// Für `--key=value` also explizit `with_separator("=")` setzen,
    /// sonst entsteht `--keyvalue` ohne Trennzeichen.
    pub separator: &'a str,
}

impl<'a> Formatting<'a> {
    pub fn with_separator(mut self, separator: &'a str) -> Self {
        self.separator = separator;
        self
    }

    pub fn with_affix(mut self, affix: AffixPair<'a>) -> Self {
        self.affix = affix;
        self
    }
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