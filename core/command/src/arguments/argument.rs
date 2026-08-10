use core::fmt;
use crate::delimited_string::{DelimitedString, Delimiter};
use crate::arguments::argument_builder::ArgumentBuilder;

use alloc::{string::{String,ToString}, vec::Vec};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Argument<'a> {
    pub prefix: Option<&'a str>,
    pub key: DelimitedString<'a>,
    pub separator: Option<&'a str>,
    pub value: Option<DelimitedString<'a>>,
    pub postfix: Option<&'a str>,
    pub delimiter: Option<Delimiter<'a>>,
}

impl<'a> Argument<'a> {
    pub fn builder() -> ArgumentBuilder<'a> {
        ArgumentBuilder::new()
    }
    
    pub fn render(&self) -> String {
        let value_len = self.value.as_ref().map_or(0, |v| v.len());
        let len = self.prefix.map_or(0, |s| s.len())
            + self.key.len()
            + self.separator.map_or(0, |s| s.len())
            + value_len
            + self.postfix.map_or(0, |s| s.len())
            + self.delimiter.map_or(0, |d| d.len());

        let mut result = String::with_capacity(len);
        use core::fmt::Write;
        let _ = write!(result, "{}", self);
        result
    }

    /// Extrahieren der un-quoted Argv-Tokens für `std::process::Command`
    pub fn to_arg_tokens(&self) -> Vec<String> {
        let mut tokens = Vec::new();
        let key_raw = self.key.value;
        let val_raw = self.value.as_ref().map(|v| v.value);

        match (self.prefix, self.separator, val_raw) {
            // z.B. -v /var/sock (Separator ist Leerzeichen -> 2 getrennte Tokens)
            (Some(p), Some(sep), Some(v)) if sep.trim().is_empty() => {
                tokens.push(alloc::format!("{}{}", p, key_raw));
                tokens.push(v.to_string());
            }
            // z.B. --env=NODE_ENV=production (Mit Separator -> 1 Token ohne Quotes)
            (Some(p), Some(sep), Some(v)) => {
                tokens.push(alloc::format!("{}{}{} {}", p, key_raw, sep, v));
            }
            // z.B. --release oder -v (Flag ohne Value)
            (Some(p), None, None) => {
                tokens.push(alloc::format!("{}{}", p, key_raw));
            }
            // z.B. Positional Argument
            (None, _, _) => {
                tokens.push(key_raw.to_string());
            }
            _ => {
                tokens.push(self.key_str().to_string());
            }
        }

        tokens
    }

    pub fn is_flag(&self) -> bool {
        self.prefix.is_some() && self.value.is_none()
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    pub fn is_positional(&self) -> bool {
        self.prefix.is_none()
    }

    pub fn is_long_flag(&self) -> bool {
        self.prefix.map_or(false, |p| p.starts_with("--"))
    }

    pub fn is_short_flag(&self) -> bool {
        self.prefix.map_or(false, |p| p == "-" || p == "/")
    }

    pub fn is_value_quoted(&self) -> bool {
        self.value.as_ref().map_or(false, |v| v.is_quoted())
    }

    pub fn key_str(&self) -> &'a str {
        self.key.value
    }

    pub fn value_str(&self) -> Option<&'a str> {
        self.value.as_ref().map(|v| v.value)
    }

    pub fn value_or(&self, default: &'a str) -> &'a str {
        self.value_str().unwrap_or(default)
    }
}

impl<'a> fmt::Display for Argument<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref d) = self.delimiter {
            if let Some(start) = d.start {
                f.write_str(start)?;
            }
        }

        if let Some(prefix) = self.prefix {
            f.write_str(prefix)?;
        }
        write!(f, "{}", self.key)?;
        if let Some(separator) = self.separator {
            f.write_str(separator)?;
        }
        if let Some(ref value) = self.value {
            write!(f, "{}", value)?;
        }
        if let Some(postfix) = self.postfix {
            f.write_str(postfix)?;
        }

        if let Some(ref d) = self.delimiter {
            if let Some(end) = d.end {
                f.write_str(end)?;
            }
        }

        Ok(())
    }
}