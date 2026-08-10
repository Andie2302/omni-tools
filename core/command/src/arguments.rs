use crate::argument::Argument;
use crate::delimited_string::Delimiter;
use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arguments<'a> {
    items: Vec<Argument<'a>>,
    pub item_separator: &'a str,
    pub delimiter: Option<Delimiter<'a>>,
}

impl<'a> Default for Arguments<'a> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            item_separator: " ",
            delimiter: None,
        }
    }
}

impl<'a> Arguments<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            item_separator: " ",
            delimiter: None,
        }
    }

    /// Ändert den Separator für die Ausgabe aller Items (Fluent API).
    pub fn with_item_separator(mut self, separator: &'a str) -> Self {
        self.item_separator = separator;
        self
    }

    /// Setzt den Separator nachträglich.
    pub fn set_item_separator(&mut self, separator: &'a str) {
        self.item_separator = separator;
    }

    /// Setzt individuelle Start- und End-Begrenzer um die gesamte Argumentenliste (Fluent API).
    pub fn with_delimiter(mut self, start: &'a str, end: &'a str) -> Self {
        self.delimiter = Some(Delimiter::new(Some(start), Some(end)));
        self
    }

    /// Fügt ein Argument zur Liste hinzu.
    pub fn push(&mut self, arg: Argument<'a>) {
        self.items.push(arg);
    }

    /// Gibt die Anzahl der Argument-Einträge zurück.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // --- Dynamische Längenberechnung & Rendering ---

    /// Berechnet die exakte String-Länge der gerenderten Argumentenliste ohne Allokation.
    pub fn rendered_len(&self) -> usize {
        if self.items.is_empty() {
            return self.delimiter.map_or(0, |d| d.len());
        }

        // 1. Länge aller Einzel-Argumente aufsummieren
        let args_len: usize = self
            .items
            .iter()
            .map(|arg| {
                let value_len = arg.value.as_ref().map_or(0, |v| v.len());
                let delimiter_len = arg.delimiter.map_or(0, |d| d.len());

                arg.prefix.map_or(0, |s| s.len())
                    + arg.key.len()
                    + arg.separator.map_or(0, |s| s.len())
                    + value_len
                    + arg.postfix.map_or(0, |s| s.len())
                    + delimiter_len
            })
            .sum();

        // 2. Gesamtlänge der Item-Separatoren dazwischen berechnen
        let separators_len = (self.items.len() - 1) * self.item_separator.len();

        // 3. Länge des äußeren Delimiters hinzurechnen
        let outer_delimiter_len = self.delimiter.map_or(0, |d| d.len());

        args_len + separators_len + outer_delimiter_len
    }

    #[cfg(feature = "alloc")]
    pub fn render(&self) -> String {
        let total_len = self.rendered_len();
        if total_len == 0 {
            return String::new();
        }

        let mut result = String::with_capacity(total_len);
        use core::fmt::Write;
        let _ = write!(result, "{}", self);
        result
    }
}

// --- Display mit Delimiter & item_separator ---

impl<'a> fmt::Display for Arguments<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 1. Äußeren Delimiter-Start schreiben (falls vorhanden)
        if let Some(ref d) = self.delimiter {
            if let Some(start) = d.start {
                f.write_str(start)?;
            }
        }

        // 2. Alle Items getrennt durch item_separator schreiben
        for (i, arg) in self.items.iter().enumerate() {
            if i > 0 {
                f.write_str(self.item_separator)?;
            }
            write!(f, "{}", arg)?;
        }

        // 3. Äußeren Delimiter-End schreiben (falls vorhanden)
        if let Some(ref d) = self.delimiter {
            if let Some(end) = d.end {
                f.write_str(end)?;
            }
        }

        Ok(())
    }
}