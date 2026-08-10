use crate::arguments::argument::Argument;
use crate::arguments::delimited_string::{DelimitedString, Delimiter};

#[derive(Debug, Default)]
pub struct ArgumentBuilder<'a> {
    prefix: Option<&'a str>,
    key: Option<DelimitedString<'a>>,
    separator: Option<&'a str>,
    value: Option<DelimitedString<'a>>,
    postfix: Option<&'a str>,
    delimiter: Option<Delimiter<'a>>,
}

impl<'a> ArgumentBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn prefix(mut self, prefix: &'a str) -> Self {
        self.prefix = Some(prefix);
        self
    }

    pub fn key(mut self, key: impl Into<DelimitedString<'a>>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn separator(mut self, separator: &'a str) -> Self {
        self.separator = Some(separator);
        self
    }

    pub fn value(mut self, value: impl Into<DelimitedString<'a>>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Hilfsmethode: Setzt ein Value in doppelte Anführungszeichen (z. B. `"value"`).
    pub fn quoted_value(mut self, value: &'a str) -> Self {
        self.value = Some(DelimitedString::new(Some("\""), value, Some("\"")));
        self
    }

    pub fn postfix(mut self, postfix: &'a str) -> Self {
        self.postfix = Some(postfix);
        self
    }

    /// Setzt individuelle Start- und End-Begrenzer um das gesamte Argument (z.B. `[` und `]`).
    pub fn delimiter(mut self, start: &'a str, end: &'a str) -> Self {
        self.delimiter = Some(Delimiter::new(Some(start), Some(end)));
        self
    }

    /// Akzeptiert direkt ein `Delimiter`-Objekt.
    pub fn with_delimiter(mut self, delimiter: Delimiter<'a>) -> Self {
        self.delimiter = Some(delimiter);
        self
    }

    pub fn build(self) -> Option<Argument<'a>> {
        let key = self.key?;
        Some(Argument {
            prefix: self.prefix,
            key,
            separator: self.separator,
            value: self.value,
            postfix: self.postfix,
            delimiter: self.delimiter,
        })
    }
}