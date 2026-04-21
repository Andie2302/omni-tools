use crate::varint::{VarInt};

// ============================================================================
// Serialize-Trait
// ============================================================================

/// Alles was diesen Trait implementiert kann sich in sein Wire-Format
/// serialisieren. Block-Structs implementieren ihn indem sie ihre
/// Felder der Reihe nach serialisieren.
pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

// ============================================================================
// ExportMode
// ============================================================================

/// Steuert was `to_bytes_ext` ausgibt.
///
/// Wire-Format von OmniData:
///   Full       →  [VarInt(len)][...len Bytes...]
///   DataOnly   →  [...len Bytes...]
///   LengthOnly →  [VarInt(len)]
pub enum ExportMode {
    /// [VarInt(len)][daten]  — das Standard-Format auf dem Wire
    Full,
    /// Nur die rohen Bytes, ohne Längen-Prefix
    DataOnly,
    /// Nur den VarInt der Datenlänge, ohne die Daten selbst
    LengthOnly,
}

// ============================================================================
// OmniData
// ============================================================================

/// Ein Byte-Puffer der sich selbst mit einer VarInt-Länge serialisieren kann.
///
/// Wire-Format (Full):  [VarInt(len)][...len Bytes...]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OmniData {
    pub data: Vec<u8>,
}

impl OmniData {
    pub fn new() -> Self {
        Self::default()
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

    /// Kurzform: Full-Modus → [VarInt(len)][daten]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes_ext(ExportMode::Full)
    }

    pub fn to_bytes_ext(&self, mode: ExportMode) -> Vec<u8> {
        match mode {
            ExportMode::DataOnly => {
                self.data.clone()
            }
            ExportMode::LengthOnly => {
                VarInt::new(self.data.len() as u64).data
            }
            ExportMode::Full => {
                let mut buf = VarInt::new(self.data.len() as u64).data;
                buf.extend_from_slice(&self.data);
                buf
            }
        }
    }
}

impl Serialize for OmniData {
    /// Serialisiert als Full-Modus: [VarInt(len)][daten]
    fn serialize(&self) -> Vec<u8> {
        self.to_bytes()
    }
}

// ============================================================================
// OmniTyp
// ============================================================================

/// Ein Typ-Tag aus einer Folge von VarInts.
///
/// Wire-Format:  [VarInt(n)][VarInt₁]..[VarIntₙ]
///
/// Bedeutung von n:
///   0  → kein VarInt folgt  (Binär-Rohformat o.ä.)
///   1  → genau 1 VarInt folgt
///   n  → n VarInts folgen
///
/// Beispiel: Typ-Code „2.7" wäre OmniTyp::new(vec![2, 7])
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OmniTyp {
    /// Die eigentlichen Typ-VarInts (ohne die Länge selbst)
    pub ids: Vec<u64>,
}

impl OmniTyp {
    pub fn new(ids: impl Into<Vec<u64>>) -> Self {
        Self { ids: ids.into() }
    }

    /// Leerer Typ (n=0): kein VarInt folgt
    pub fn empty() -> Self {
        Self::default()
    }

    /// Anzahl der Typ-VarInts
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }
}

impl Serialize for OmniTyp {
    /// Wire-Format: [VarInt(n)][VarInt₁]..[VarIntₙ]
    fn serialize(&self) -> Vec<u8> {
        // Anzahl als VarInt voranstellen
        let mut buf = VarInt::new(self.ids.len() as u64).data;
        // Danach jeden Typ-VarInt
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

impl Serialize for OmniHeader {
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

impl Serialize for OmniContent {
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

impl Serialize for OmniFooter {
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
}

impl OmniBlock {
    pub fn new(typ: OmniTyp, data: OmniData) -> Self {
        Self {
            header:  OmniHeader::new(typ),
            content: OmniContent::new(data),
        }
    }
}

impl Serialize for OmniBlock {
    fn serialize(&self) -> Vec<u8> {
        let mut buf = self.header.serialize();
        buf.extend(self.content.serialize());
        buf
    }
}

// ============================================================================
// OmniHashBlock  (Block mit Hash-Footer)
// ============================================================================

/// Ein Block mit optionalem Hash im Footer.
///
/// Wire-Format:
///   [OmniHeader][OmniContent][OmniFooter]
///   = ...[VarInt(hash_len)][hash_bytes]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OmniHashBlock {
    pub header:  OmniHeader,
    pub content: OmniContent,
    pub footer:  OmniFooter,
}

impl OmniHashBlock {
    pub fn new(typ: OmniTyp, data: OmniData, hash: OmniData) -> Self {
        Self {
            header:  OmniHeader::new(typ),
            content: OmniContent::new(data),
            footer:  OmniFooter::new(hash),
        }
    }
}

impl Serialize for OmniHashBlock {
    fn serialize(&self) -> Vec<u8> {
        let mut buf = self.header.serialize();
        buf.extend(self.content.serialize());
        buf.extend(self.footer.serialize());
        buf
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use crate::varint::FromVarint;
    use super::*;

    // --- OmniData -----------------------------------------------------------

    #[test]
    fn omnidata_full_modus() {
        let d = OmniData::from_bytes(vec![0xAA, 0xBB, 0xCC]);
        let bytes = d.to_bytes();
        // Erstes Byte muss VarInt(3) = 0x03 sein
        assert_eq!(bytes[0], 0x03);
        assert_eq!(&bytes[1..], &[0xAA, 0xBB, 0xCC]);
    }

    #[test]
    fn omnidata_data_only() {
        let d = OmniData::from_bytes(vec![1, 2, 3]);
        assert_eq!(d.to_bytes_ext(ExportMode::DataOnly), vec![1, 2, 3]);
    }

    #[test]
    fn omnidata_length_only() {
        let d = OmniData::from_bytes(vec![1, 2, 3]);
        let lb = d.to_bytes_ext(ExportMode::LengthOnly);
        let (len, _) = u64::from_varint(&lb).unwrap();
        assert_eq!(len, 3);
    }

    // --- OmniTyp ------------------------------------------------------------

    #[test]
    fn omnitype_leer() {
        // n=0 → nur ein einziges 0x00-Byte
        let t = OmniTyp::empty();
        assert_eq!(t.serialize(), vec![0x00]);
    }

    #[test]
    fn omnitype_ein_id() {
        // n=1, id=42
        let t = OmniTyp::new(vec![42]);
        let bytes = t.serialize();
        assert_eq!(bytes[0], 0x01); // VarInt(1)
        assert_eq!(bytes[1], 0x2A); // VarInt(42) = 0x2A
    }

    #[test]
    fn omnitype_mehrere_ids() {
        // n=3, ids=[1,2,3]
        let t = OmniTyp::new(vec![1, 2, 3]);
        let bytes = t.serialize();
        assert_eq!(bytes[0], 0x03); // VarInt(3)
        assert_eq!(&bytes[1..], &[0x01, 0x02, 0x03]);
    }

    // --- OmniBlock ----------------------------------------------------------

    #[test]
    fn omniblock_wire_format() {
        let typ  = OmniTyp::new(vec![1]);
        let data = OmniData::from_bytes(vec![0xFF, 0xFE]);
        let block = OmniBlock::new(typ, data);
        let bytes = block.serialize();

        // [VarInt(1)=0x01][VarInt(1)=0x01]  ← Header: 1 id, id=1
        // [VarInt(2)=0x02][0xFF][0xFE]       ← Content
        assert_eq!(bytes, vec![0x01, 0x01, 0x02, 0xFF, 0xFE]);
    }

    // --- OmniHashBlock ------------------------------------------------------

    #[test]
    fn omnihashblock_enthaelt_footer() {
        let typ  = OmniTyp::new(vec![2, 7]);
        let data = OmniData::from_bytes(vec![0xAA]);
        let hash = OmniData::from_bytes(vec![0x01, 0x02]); // Fake-Hash
        let block = OmniHashBlock::new(typ, data, hash);
        let bytes = block.serialize();

        // Header:  [VarInt(2)][VarInt(2)][VarInt(7)]
        // Content: [VarInt(1)][0xAA]
        // Footer:  [VarInt(2)][0x01][0x02]
        assert_eq!(bytes, vec![
            0x02, 0x02, 0x07, // Header: n=2, ids=[2,7]
            0x01, 0xAA,       // Content: len=1, data=0xAA
            0x02, 0x01, 0x02, // Footer: len=2, hash=[0x01,0x02]
        ]);
    }
}