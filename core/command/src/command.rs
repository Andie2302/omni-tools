use crate::arguments::argument::Argument;
use alloc::string::String;


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

    /// Berechnet die exakte Zeichenlänge des fertigen Kommandos (Pfad + Leerzeichen + Argumente).
    pub fn len(&self) -> usize {
        if self.arguments.is_empty() {
            self.path.len()
        } else {
            // Nutzt rendered_len() statt render().len() -> Keine Allokation & exakte Länge!
            self.path.len() + 1 + self.arguments.rendered_len()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    #[cfg(feature = "alloc")]
    pub fn render(&self) -> String {
        let mut result = String::with_capacity(self.len());

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