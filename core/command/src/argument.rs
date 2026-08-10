use crate::delimiter::{
    DelimiterKey, DelimiterPostfix, DelimiterPrefix, DelimiterSeparator, DelimiterString,
    DelimiterValue,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentError {
    EmptySeparator,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueSeparator<'a> {
    pub separator: DelimiterString<'a, DelimiterSeparator>,
    pub value: DelimiterString<'a, DelimiterValue>,
}

impl<'a> ValueSeparator<'a> {
    pub fn new(
        separator: DelimiterString<'a, DelimiterSeparator>,
        value: DelimiterString<'a, DelimiterValue>,
    ) -> Result<Self, ArgumentError> {
        if separator.delimit().is_empty() {
            return Err(ArgumentError::EmptySeparator);
        }
        Ok(Self { separator, value })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Argument<'a> {
    pub key: DelimiterString<'a, DelimiterKey>,
    pub prefix: Option<DelimiterString<'a, DelimiterPrefix>>,
    pub value_separator: Option<ValueSeparator<'a>>,
    pub postfix: Option<DelimiterString<'a, DelimiterPostfix>>,
    pub enclosing_delimiter: Option<(&'a str, &'a str)>,
}

impl<'a> Argument<'a> {
    pub fn new(
        key: DelimiterString<'a, DelimiterKey>,
        prefix: Option<DelimiterString<'a, DelimiterPrefix>>,
        value_separator: Option<ValueSeparator<'a>>,
        postfix: Option<DelimiterString<'a, DelimiterPostfix>>,
    ) -> Self {
        Self {
            key,
            prefix,
            value_separator,
            postfix,
            enclosing_delimiter: None,
        }
    }

    /// Hilfskonstruktor für einfache Flags (z.B. "-y" oder "--verbose")
    pub fn flag(key: &'a str, prefix: &'a str) -> Self {
        Self::new(
            DelimiterString::new(key, None, None),
            Some(DelimiterString::new(prefix, None, None)),
            None,
            None,
        )
    }

    /// Hilfskonstruktor für Key-Value Paare (z.B. "--file kdeglobals" oder "app=gimp")
    pub fn key_value(key: &'a str, prefix: &'a str, separator: &'a str, value: &'a str) -> Self {
        let vs = ValueSeparator::new(
            DelimiterString::new(separator, None, None),
            DelimiterString::new(value, None, None),
        )
            .expect("Separator darf nicht leer sein");

        Self::new(
            DelimiterString::new(key, None, None),
            if prefix.is_empty() {
                None
            } else {
                Some(DelimiterString::new(prefix, None, None))
            },
            Some(vs),
            None,
        )
    }

    pub fn with_enclosing_delimiter(mut self, start: &'a str, end: &'a str) -> Self {
        self.enclosing_delimiter = Some((start, end));
        self
    }

    pub fn render(&self) -> String {
        let mut raw = String::new();

        if let Some(ref p) = self.prefix {
            raw.push_str(&p.delimit());
        }

        raw.push_str(&self.key.delimit());

        if let Some(ref vs) = self.value_separator {
            raw.push_str(&vs.separator.delimit());
            raw.push_str(&vs.value.delimit());
        }

        if let Some(ref post) = self.postfix {
            raw.push_str(&post.delimit());
        }

        if let Some((start, end)) = self.enclosing_delimiter {
            format!("{start}{raw}{end}")
        } else {
            raw
        }
    }
}