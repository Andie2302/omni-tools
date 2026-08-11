use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AffixPair<'a> {
    pub prefix: &'a str,
    pub postfix: &'a str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AffixedString<'a> {
    pub text: &'a str,
    pub affix_pair: AffixPair<'a>,
}

impl<'a> AffixedString<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            affix_pair: Default::default(),
        }
    }

    /// Setzt Prefix und Postfix auf denselben Wert.
    ///
    /// Konsumiert `self`, damit direktes Chaining ab `new(...)` möglich ist:
    /// `AffixedString::new("x").with_affix("[")`.
    pub fn with_affix(mut self, affix: &'a str) -> Self {
        self.affix_pair.prefix = affix;
        self.affix_pair.postfix = affix;
        self
    }

    /// Setzt Prefix und Postfix getrennt.
    pub fn with_separate_affix(mut self, prefix: &'a str, postfix: &'a str) -> Self {
        self.affix_pair.prefix = prefix;
        self.affix_pair.postfix = postfix;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

impl<'a> fmt::Display for AffixedString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.affix_pair.prefix, self.text, self.affix_pair.postfix
        )
    }
}