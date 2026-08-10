use core::fmt;
use crate::argument_builder::ArgumentBuilder;
use crate::delimited_string::{DelimitedString, Delimiter};

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
        use std::fmt::Write;
        let _ = write!(result, "{}", self);
        result
    }

    // --- Zustands-Abfragen ---

    /// Prüft, ob das Argument ein Flag/Schalter ist (hat Prefix, aber kein Value).
    pub fn is_flag(&self) -> bool {
        self.prefix.is_some() && self.value.is_none()
    }

    /// Prüft, ob ein Value vorhanden ist.
    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    /// Prüft, ob es sich um ein positional Argument handelt (kein Prefix).
    pub fn is_positional(&self) -> bool {
        self.prefix.is_none()
    }

    /// Prüft, ob das Argument mit "--" beginnt.
    pub fn is_long_flag(&self) -> bool {
        self.prefix.map_or(false, |p| p.starts_with("--"))
    }

    /// Prüft, ob das Argument ein kurzes Flag ist ("-" oder "/").
    pub fn is_short_flag(&self) -> bool {
        self.prefix.map_or(false, |p| p == "-" || p == "/")
    }

    /// Prüft, ob der Value in Anführungszeichen gefasst ist.
    pub fn is_value_quoted(&self) -> bool {
        self.value.as_ref().map_or(false, |v| v.is_quoted())
    }

    // --- Convenience-Getter ---

    /// Gibt den reinen Hauptwert des Keys als `&str` zurück.
    pub fn key_str(&self) -> &'a str {
        self.key.value
    }

    /// Gibt den reinen Hauptwert des Values als `&str` zurück, falls vorhanden.
    pub fn value_str(&self) -> Option<&'a str> {
        self.value.as_ref().map(|v| v.value)
    }

    /// Gibt das Value zurück oder einen Fallback-Wert.
    pub fn value_or(&self, default: &'a str) -> &'a str {
        self.value_str().unwrap_or(default)
    }
}

impl<'a> fmt::Display for Argument<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 1. Wenn vorhanden: Outer Delimiter Start schreiben
        if let Some(ref d) = self.delimiter {
            if let Some(start) = d.start {
                f.write_str(start)?;
            }
        }

        // 2. Argument-Inhalt schreiben
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

        // 3. Wenn vorhanden: Outer Delimiter End schreiben
        if let Some(ref d) = self.delimiter {
            if let Some(end) = d.end {
                f.write_str(end)?;
            }
        }

        Ok(())
    }
}

// ==========================================
// Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_flag() {
        let arg = Argument::builder()
            .prefix("-")
            .key("v")
            .build()
            .expect("Key war angegeben");

        assert!(arg.is_flag());
        assert!(arg.is_short_flag());
        assert!(!arg.has_value());
        assert_eq!(arg.render(), "-v");
    }

    #[test]
    fn test_quoted_value() {
        let arg = Argument::builder()
            .prefix("--")
            .key("path")
            .separator("=")
            .quoted_value("/usr/local/bin")
            .build()
            .expect("Key war angegeben");

        assert!(arg.has_value());
        assert!(arg.is_value_quoted());
        assert_eq!(arg.value_str(), Some("/usr/local/bin"));
        assert_eq!(arg.render(), "--path=\"/usr/local/bin\"");
    }

    #[test]
    fn test_outer_delimiter() {
        let arg = Argument::builder()
            .delimiter("[", "]")
            .prefix("--")
            .key("env")
            .separator("=")
            .value("production")
            .build()
            .unwrap();

        assert_eq!(arg.render(), "[--env=production]");
    }

    #[test]
    fn test_capacity_and_render_length() {
        let arg = Argument::builder()
            .prefix("--")
            .key("env")
            .separator("=")
            .value("production")
            .postfix(";")
            .build()
            .unwrap();

        let rendered = arg.render();
        assert_eq!(rendered, "--env=production;");
        assert_eq!(rendered.len(), 17);
    }
}