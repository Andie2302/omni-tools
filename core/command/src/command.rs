use core::fmt;
use crate::arguments::Arguments;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command<'a> {
    pub path: &'a str,
    pub arguments: Arguments<'a>,
}

impl<'a> Command<'a> {
    pub fn new(path: &'a str, arguments: Arguments<'a>) -> Self {
        Self { path, arguments }
    }

    pub fn plain(path: &'a str) -> Self {
        Self {
            path,
            arguments: Arguments::new(),
        }
    }

    /// Berechnet die exakte Gesamtlänge für die Pufferallokation.
    pub fn len(&self) -> usize {
        if self.arguments.is_empty() {
            self.path.len()
        } else {
            // Pfadlänge + 1 (Leerzeichen) + Länge der gerenderten Argumente
            self.path.len() + 1 + self.arguments.render().len()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    /// Rendert das gesamte Kommando inkl. Pfad und Argumenten in einen String.
    pub fn render(&self) -> String {
        let mut result = String::with_capacity(self.len());

        // Pfad direkt in den String schreiben
        result.push_str(self.path);

        if !self.arguments.is_empty() {
            result.push(' ');
            use core::fmt::Write;
            let _ = write!(result, "{}", self.arguments);
        }

        result
    }
}

impl<'a> fmt::Display for Command<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.path)?;

        if !self.arguments.is_empty() {
            f.write_str(" ")?;
            fmt::Display::fmt(&self.arguments, f)?;
        }

        Ok(())
    }
}