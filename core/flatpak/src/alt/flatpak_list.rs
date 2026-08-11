use crate::flatpak::Flatpak;
use command::{Argument, Arguments, Command};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlatpakListType {
    All,
    User,
    System,
    App,
    Runtime,
}

impl FlatpakListType {
    pub fn as_flag(&self) -> &'static str {
        match self {
            Self::All => "--all",
            Self::User => "--user",
            Self::System => "--system",
            Self::App => "--app",
            Self::Runtime => "--runtime",
        }
    }
}

/// Dein eigener Trait für die Konvertierung
pub trait IntoFlatpakListOptions {
    fn into_options(self) -> Vec<FlatpakListType>;
}

// 1. Einzelner Wert: FlatpakListType::User
impl IntoFlatpakListOptions for FlatpakListType {
    fn into_options(self) -> Vec<FlatpakListType> {
        vec![self]
    }
}

// 2. Slice: &[FlatpakListType::User, FlatpakListType::App]
impl IntoFlatpakListOptions for &[FlatpakListType] {
    fn into_options(self) -> Vec<FlatpakListType> {
        self.to_vec()
    }
}

// 3. Fixed-size Array: [FlatpakListType::User, FlatpakListType::App]
impl<const N: usize> IntoFlatpakListOptions for [FlatpakListType; N] {
    fn into_options(self) -> Vec<FlatpakListType> {
        self.to_vec()
    }
}

// 4. Bereits als Vec vorhanden
impl IntoFlatpakListOptions for Vec<FlatpakListType> {
    fn into_options(self) -> Vec<FlatpakListType> {
        self
    }
}

impl Flatpak {
    pub fn list<I>(options: I) -> Result<Vec<String>, std::io::Error>
    where
        I: IntoFlatpakListOptions,
    {
        let options = options.into_options();

        let mut args = Arguments::new();
        args.push(Argument::from("list"));
        args.push(Argument::from("--columns=application"));

        for opt in options {
            args.push(Argument::from(opt.as_flag()));
        }

        let output = Command::new("flatpak", args).exec_read()?;
        let apps = output
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        Ok(apps)
    }
}