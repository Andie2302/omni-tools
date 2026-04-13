use std::fmt;
use std::io;

#[derive(Debug)]
pub enum OmniError {
    /// I/O Fehler vom Betriebssystem
    Io(io::Error),
    /// LEB128 Varint würde u64 überlaufen
    VarintOverflow,
    /// Unerwartetes Dateiende beim Lesen
    UnexpectedEof,
    /// Kein gültiger Index-Footer gefunden
    NoIndexFound,
    /// Magic Number stimmt nicht überein
    InvalidMagic,
    /// Offset von 0 ist nicht erlaubt (wäre 0x00-Kollision)
    InvalidOffset,
    /// Block-Länge überschreitet verfügbare Daten
    BlockTooLarge,
    /// Typ-Array ist leer (mindestens ein Typ-Varint erforderlich)
    EmptyBlockType,
}

impl fmt::Display for OmniError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OmniError::Io(e)         => write!(f, "I/O Fehler: {e}"),
            OmniError::VarintOverflow => write!(f, "Varint Overflow: Wert überschreitet u64"),
            OmniError::UnexpectedEof  => write!(f, "Unerwartetes Dateiende"),
            OmniError::NoIndexFound   => write!(f, "Kein Index-Footer gefunden"),
            OmniError::InvalidMagic   => write!(f, "Ungültige Magic Number – keine Omni-Datei"),
            OmniError::InvalidOffset  => write!(f, "Offset 0 ist nicht erlaubt"),
            OmniError::BlockTooLarge  => write!(f, "Block-Länge überschreitet Puffergröße"),
            OmniError::EmptyBlockType => write!(f, "Block-Typ darf nicht leer sein"),
        }
    }
}

impl std::error::Error for OmniError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            OmniError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for OmniError {
    fn from(e: io::Error) -> Self {
        // UnexpectedEof als eigene Variante behandeln
        if e.kind() == io::ErrorKind::UnexpectedEof {
            OmniError::UnexpectedEof
        } else {
            OmniError::Io(e)
        }
    }
}