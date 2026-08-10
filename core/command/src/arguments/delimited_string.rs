use core::fmt;

// ==========================================
// Delimiter
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Delimiter<'a> {
    pub start: Option<&'a str>,
    pub end: Option<&'a str>,
}
impl<'a> Delimiter<'a> {
    pub fn new(start: Option<&'a str>, end: Option<&'a str>) -> Self {
        Self { start, end }
    }

    /// Hilfsmethode für symmetrische Begrenzer (z. B. doppelte Anführungszeichen)
    pub fn symmetrical(delimiter: &'a str) -> Self {
        Self {
            start: Some(delimiter),
            end: Some(delimiter),
        }
    }

    /// Berechnet die Gesamtlänge der Begrenzer
    pub fn len(&self) -> usize {
        self.start.map_or(0, |s| s.len()) + self.end.map_or(0, |s| s.len())
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Prüft, ob es sich um Anführungszeichen (doppelt oder einfach) handelt
    pub fn is_quote(&self) -> bool {
        matches!((self.start, self.end), (Some("\""), Some("\"")) | (Some("'"), Some("'")))
    }
}

// ==========================================
// DelimitedString
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DelimitedString<'a> {
    pub value: &'a str,
    pub delimiter: Option<Delimiter<'a>>,
}

impl<'a> DelimitedString<'a> {
    pub fn plain(value: &'a str) -> Self {
        Self {
            value,
            delimiter: None,
        }
    }

    pub fn new(start: Option<&'a str>, value: &'a str, end: Option<&'a str>) -> Self {
        let delimiter = if start.is_some() || end.is_some() {
            Some(Delimiter::new(start, end))
        } else {
            None
        };

        Self { value, delimiter }
    }

    pub fn with_delimiter(value: &'a str, delimiter: Delimiter<'a>) -> Self {
        Self {
            value,
            delimiter: Some(delimiter),
        }
    }

    pub fn len(&self) -> usize {
        self.value.len() + self.delimiter.map_or(0, |d| d.len())
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Prüft, ob der String in Anführungszeichen gefasst ist
    pub fn is_quoted(&self) -> bool {
        self.delimiter.map_or(false, |d| d.is_quote())
    }
}

// --- Display Implementierungen ---

impl<'a> fmt::Display for DelimitedString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref d) = self.delimiter {
            if let Some(start) = d.start {
                f.write_str(start)?;
            }
            f.write_str(self.value)?;
            if let Some(end) = d.end {
                f.write_str(end)?;
            }
        } else {
            f.write_str(self.value)?;
        }
        Ok(())
    }
}

impl<'a> From<&'a str> for DelimitedString<'a> {
    fn from(s: &'a str) -> Self {
        DelimitedString::plain(s)
    }
}