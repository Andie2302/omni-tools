use crate::varint::{VarInt};

pub trait OmniSerialize {
    fn serialize(&self) -> Vec<u8>;
}
pub enum OmniExportMode {
    Full,
    DataOnly,
    LengthOnly,
}

pub struct OmniData {
    pub data: Vec<u8>,
}

impl OmniData {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self { data: bytes.into() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes_ext(OmniExportMode::Full)
    }

    pub fn to_bytes_ext(&self, mode: OmniExportMode) -> Vec<u8> {
        match mode {
            OmniExportMode::DataOnly => {
                self.data.clone()
            }
            OmniExportMode::LengthOnly => {
                // Nutzt From<usize> (via Makro) und wandelt es in Vec<u8> um
                VarInt::from(self.data.len()).to_vec()
            }
            OmniExportMode::Full => {
                // Erzeugt VarInt aus der Länge
                let len_varint = VarInt::from(self.data.len());

                // Da VarInt Deref implementiert, können wir es direkt
                // als &[u8] an den Buffer hängen
                let mut buf = Vec::with_capacity(len_varint.len() + self.data.len());
                buf.extend_from_slice(&len_varint);
                buf.extend_from_slice(&self.data);
                buf
            }
        }
    }
}

impl OmniSerialize for OmniData {
    fn serialize(&self) -> Vec<u8> {
        self.to_bytes()
    }
}


/*
//##########################################
//##########################################
//###############          #################
//###############          #################
//###############          #################
//###############          #################
//###############          #################
//##########                    ############
//############                ##############
//#############             ################
//###############         ##################
//#################     ####################
//################### ######################
//##########################################
//##########################################



pub struct OmniTyp {
    pub ids: Vec<u64>,
}

impl OmniTyp {
    pub fn new(ids: impl Into<Vec<u64>>) -> Self {
        Self { ids: ids.into() }
    }
    pub fn empty() -> Self {
        Self::new(Vec::new())
    }
    pub fn len(&self) -> usize {
        self.ids.len()
    }
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }
}

impl OmniSerialize for OmniTyp {
    fn serialize(&self) -> Vec<u8> {
        let mut buf = VarInt::new(self.ids.len() as u64).data;
        for &id in &self.ids {
            buf.extend(VarInt::new(id).data);
        }
        buf
    }
}

// ============================================================================
// OmniHeader, OmniContent, OmniFooter
// ============================================================================

/// Hält den OmniTyp des Blocks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmniHeader {
    pub typ: OmniTyp,
}

impl OmniHeader {
    pub fn new(typ: OmniTyp) -> Self {
        Self { typ }
    }
}

impl OmniSerialize for OmniHeader {
    fn serialize(&self) -> Vec<u8> {
        self.typ.serialize()
    }
}

/// Hält die eigentlichen Nutz-Daten des Blocks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmniContent {
    pub data: OmniData,
}

impl OmniContent {
    pub fn new(data: OmniData) -> Self {
        Self { data }
    }

    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self::new(OmniData::from_bytes(bytes))
    }
}

impl OmniSerialize for OmniContent {
    fn serialize(&self) -> Vec<u8> {
        self.data.serialize()
    }
}

/// Optionaler Anhang, z.B. ein Hash. Nur in OmniHashBlock vorhanden.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmniFooter {
    pub data: OmniData,
}

impl OmniFooter {
    pub fn new(data: OmniData) -> Self {
        Self { data }
    }

    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self::new(OmniData::from_bytes(bytes))
    }
}

impl OmniSerialize for OmniFooter {
    fn serialize(&self) -> Vec<u8> {
        self.data.serialize()
    }
}

// ============================================================================
// OmniBlock  (Basis-Block, kein Footer)
// ============================================================================

/// Ein vollständiger Block ohne Footer.
///
/// Wire-Format:
///   [OmniHeader][OmniContent]
///   = [VarInt(n)][VarInt₁..n][VarInt(data_len)][data_bytes]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmniBlock {
    pub header:  OmniHeader,
    pub content: OmniContent,
    pub footer:  Option<OmniFooter>,
}

impl OmniBlock {
    pub fn new(typ: OmniTyp, data: OmniData) -> Self {
        Self {
            header:  OmniHeader::new(typ),
            content: OmniContent::new(data),

        }
    }
}

impl OmniSerialize for OmniBlock {
    fn serialize(&self) -> Vec<u8> {
        let mut buf = self.header.serialize();
        buf.extend(self.content.serialize());
        buf
    }
}

*/