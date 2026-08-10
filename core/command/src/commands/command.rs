use alloc::string::String;
use core::fmt;
use crate::arguments::arguments::Arguments;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command<'a> {
    pub path: &'a str,
    pub arguments: Arguments<'a>,
    pub current_dir: Option<&'a str>,
}

impl<'a> Command<'a> {
    pub fn new(path: &'a str, arguments: Arguments<'a>) -> Self {
        Self {
            path,
            arguments,
            current_dir: None,
        }
    }

    pub fn plain(path: &'a str) -> Self {
        Self {
            path,
            arguments: Arguments::new(),
            current_dir: None,
        }
    }

    /// Builder-Methode zum Hinzufügen/Setzen eines Ausführungsverzeichnisses.
    pub fn current_dir(mut self, dir: &'a str) -> Self {
        self.current_dir = Some(dir);
        self
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

// -----------------------------------------------------------------------------
// std-Konvertierung (geschützt über cfg(feature = "std"))
// -----------------------------------------------------------------------------

#[cfg(feature = "std")]
impl<'a> From<Command<'a>> for std::process::Command {
    fn from(cmd: Command<'a>) -> std::process::Command {
        let mut std_cmd = std::process::Command::new(cmd.path);

        if let Some(dir) = cmd.current_dir {
            std_cmd.current_dir(dir);
        }

        // Falls Arguments<'a> das `IntoIterator<Item = &str>`-Interface implementiert:
        std_cmd.args(cmd.arguments);

        /*
        // ALTERNATIVE: Falls Arguments kein Iterator ist, aber z. B. `iter()` oder Slices bietet:
        // std_cmd.args(cmd.arguments.as_slice());
        //
        // ODER falls Arguments nur ein Display/Render-Feature hat:
        // if !cmd.arguments.is_empty() {
        //     std_cmd.arg(cmd.arguments.to_string());
        // }
        */

        std_cmd
    }
}

#[cfg(feature = "std")]
impl<'a> From<&Command<'a>> for std::process::Command {
    fn from(cmd: &Command<'a>) -> std::process::Command {
        let mut std_cmd = std::process::Command::new(cmd.path);

        if let Some(dir) = cmd.current_dir {
            std_cmd.current_dir(dir);
        }

        // Falls &Arguments<'a> iterierbar über &str ist:
        std_cmd.args(&cmd.arguments);

        std_cmd
    }
}