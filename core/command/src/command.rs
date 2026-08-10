use core::fmt;
use crate::arguments::Arguments;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command<'a> {
    pub path: &'a str,
    pub arguments: Arguments<'a>,
}

impl<'a> Command<'a> {
    /// Erstellt ein neues `Command` mit Pfad und Argumenten.
    pub fn new(path: &'a str, arguments: Arguments<'a>) -> Self {
        Self { path, arguments }
    }

    /// Erstellt ein `Command` ohne Argumente.
    pub fn plain(path: &'a str) -> Self {
        Self {
            path,
            arguments: Arguments::new(),
        }
    }

    /// Berechnet die exakte String-Länge für die gepufferte Ausgabe.
    pub fn len(&self) -> usize {
        if self.arguments.is_empty() {
            self.path.len()
        } else {
            // Pfad + Leerzeichen/Separator-Abstand + Argumente
            self.path.len() + 1 + self.arguments.render().len()
        }
    }

    /// Prüft, ob der Pfad leer ist.
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    /// Rendert das gesamte Kommando in ein `String` (erfordert `alloc` / `std`).
    #[cfg(feature = "alloc")]
    pub fn render(&self) -> String {
        let mut result = String::with_capacity(self.len());
        use core::fmt::Write;
        let _ = write!(result, "{}", self);
        result
    }
}

impl<'a> fmt::Display for Command<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.path)?;

        if !self.arguments.is_empty() {
            f.write_str(" ")?;
            write!(f, "{}", self.arguments)?;
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
    use crate::argument::Argument;

    #[test]
    fn test_plain_command() {
        let cmd = Command::plain("ls");
        assert_eq!(cmd.path, "ls");
        assert!(cmd.arguments.is_empty());
        assert_eq!(cmd.arguments.render(), "ls");
    }

    #[test]
    fn test_command_with_arguments() {
        let mut args = Arguments::new();
        args.push(
            Argument::builder()
                .prefix("-")
                .key("l")
                .build()
                .unwrap(),
        );
        args.push(
            Argument::builder()
                .prefix("-")
                .key("a")
                .build()
                .unwrap(),
        );

        let cmd = Command::new("ls", args);

        assert_eq!(cmd.arguments.render(), "ls -l -a");
        assert_eq!(cmd.len(), 8);
    }
}