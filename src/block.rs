use crate::error::OmniError;
use crate::varint::{decode_varint, encode_varint, read_varint, write_varint};
use smallvec::SmallVec;
use std::io::{Read, Seek, SeekFrom, Write};

/// Magic Number: [0x04][OMNI]
/// Länge 4, dann die 4 ASCII-Bytes 'O','M','N','I'
pub const MAGIC: &[u8] = &[0x04, 0x4F, 0x4D, 0x4E, 0x49];

/// Bis zu 8 Typ-Varints auf dem Stack, danach Heap.
/// Für normale Blöcke reichen 1–3 Werte völlig aus.
pub type BlockTypeVec = SmallVec<u64, 8>;

/// Der Typ eines Blocks – eine geordnete Liste von Varints.
///
/// Beispiel Daten-Block:   [1, namespace, typ_id]
/// Beispiel Checksum:      [2, algo_id]
/// Beispiel Compression:   [3, algo_id]
/// Beispiel Encryption:    [4, algo_id]
/// Beispiel Meta:          [5]
/// Beispiel Index:         [6]
#[derive(Debug, Clone, PartialEq)]
pub struct BlockType(pub BlockTypeVec);

impl BlockType {
    pub fn new(values: impl IntoIterator<Item = u64>) -> Result<Self, OmniError> {
        let vec: BlockTypeVec = values.into_iter().collect();
        if vec.is_empty() {
            return Err(OmniError::EmptyBlockType);
        }
        Ok(BlockType(vec))
    }

    pub fn primary(&self) -> u64 {
        self.0[0]
    }

    /// Serialisiert: [Anzahl als Varint][Varint₁][Varint₂]...[VarintN]
    pub fn encode(&self) -> Vec<u8> {
        let mut out = encode_varint(self.0.len() as u64);
        for &v in &self.0 {
            out.extend(encode_varint(v));
        }
        out
    }

    /// Deserialisiert aus einem Byte-Slice.
    /// Gibt (BlockType, bytes_verbraucht) zurück.
    pub fn decode(bytes: &[u8]) -> Result<(Self, usize), OmniError> {
        let (count, mut pos) = decode_varint(bytes)?;
        if count == 0 {
            return Err(OmniError::EmptyBlockType);
        }
        let mut vec = BlockTypeVec::new();
        for _ in 0..count {
            let (val, n) = decode_varint(&bytes[pos..])?;
            vec.push(val);
            pos += n;
        }
        Ok((BlockType(vec), pos))
    }

    /// Liest einen BlockType aus einem Reader.
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, OmniError> {
        let (count, _) = read_varint(reader)?;
        if count == 0 {
            return Err(OmniError::EmptyBlockType);
        }
        let mut vec = BlockTypeVec::new();
        for _ in 0..count {
            let (val, _) = read_varint(reader)?;
            vec.push(val);
        }
        Ok(BlockType(vec))
    }

    /// Schreibt einen BlockType in einen Writer.
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<usize, OmniError> {
        let mut total = write_varint(writer, self.0.len() as u64)?;
        for &v in &self.0 {
            total += write_varint(writer, v)?;
        }
        Ok(total)
    }
}

/// Ein vollständiger Block-Header (ohne Daten).
#[derive(Debug, Clone)]
pub struct BlockHeader {
    /// Gesamtlänge der Daten nach dem Header (Typ + Payload)
    pub data_len: u64,
    /// Der Typ des Blocks
    pub block_type: BlockType,
}

impl BlockHeader {
    /// Schreibt [Länge][Typ] und gibt die Gesamtgröße zurück.
    ///
    /// Wichtig: data_len muss die Länge von (Typ-Bytes + Payload) sein.
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<(), OmniError> {
        write_varint(writer, self.data_len)?;
        self.block_type.write(writer)?;
        Ok(())
    }

    /// Liest [Länge][Typ] aus einem Reader.
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, OmniError> {
        let (data_len, _) = read_varint(reader)?;
        let block_type = BlockType::read(reader)?;
        Ok(BlockHeader { data_len, block_type })
    }
}

/// Ein vollständiger Block im Speicher.
#[derive(Debug, Clone)]
pub struct Block {
    pub block_type: BlockType,
    pub payload: Vec<u8>,
}

impl Block {
    pub fn new(block_type: BlockType, payload: Vec<u8>) -> Self {
        Block { block_type, payload }
    }

    /// Schreibt den vollständigen Block: [Länge][Typ][Payload]
    ///
    /// Die Länge umfasst Typ-Bytes + Payload-Bytes.
    pub fn write<W: Write>(&self, writer: &mut W) -> Result<usize, OmniError> {
        let type_bytes = self.block_type.encode();
        let data_len = (type_bytes.len() + self.payload.len()) as u64;

        let mut total = write_varint(writer, data_len)?;
        writer.write_all(&type_bytes)?;
        total += type_bytes.len();
        writer.write_all(&self.payload)?;
        total += self.payload.len();
        Ok(total)
    }

    /// Liest einen vollständigen Block aus einem Reader.
    pub fn read<R: Read>(reader: &mut R) -> Result<Self, OmniError> {
        let (data_len, _) = read_varint(reader)?;
        if data_len == 0 {
            return Err(OmniError::EmptyBlockType);
        }

        // Lese data_len Bytes in einen Puffer
        let mut data = vec![0u8; data_len as usize];
        reader.read_exact(&mut data)?;

        // Typ aus dem Puffer dekodieren
        let (block_type, type_size) = BlockType::decode(&data)?;
        let payload = data[type_size..].to_vec();

        Ok(Block { block_type, payload })
    }
}

/// Skip-Funktion: überspringt einen Block ohne ihn zu lesen.
/// Funktioniert für jeden Block-Typ – auch unbekannte.
///
/// Gibt die Gesamtzahl übersprungener Bytes zurück
/// (inklusive Längen-Varint selbst).
pub fn skip_block<R: Read + Seek>(reader: &mut R) -> Result<u64, OmniError> {
    let _start = reader.stream_position()?;
    let (data_len, len_bytes) = read_varint(reader)?;
    reader.seek(SeekFrom::Current(data_len as i64))?;
    Ok(len_bytes as u64 + data_len)
}

// NEU – nur Typ lesen, Payload via seek überspringen
pub fn read_header_skip_payload<R: Read + Seek>(
    reader: &mut R,
) -> Result<BlockHeader, OmniError> {
    let (data_len, _) = read_varint(reader)?;
    let payload_start = reader.stream_position()?;

    // Typ lesen (variable Anzahl Varints)
    let block_type = BlockType::read(reader)?;

    // Payload überspringen ohne ihn zu lesen
    let type_bytes_read = reader.stream_position()? - payload_start;
    let payload_len = data_len.saturating_sub(type_bytes_read);
    reader.seek(SeekFrom::Current(payload_len as i64))?;

    Ok(BlockHeader { data_len, block_type })
}

/// Schreibt die Magic Number an den Anfang.
pub fn write_magic<W: Write>(writer: &mut W) -> Result<(), OmniError> {
    writer.write_all(MAGIC)?;
    Ok(())
}

/// Prüft ob die Magic Number am Anfang korrekt ist.
pub fn check_magic<R: Read>(reader: &mut R) -> Result<(), OmniError> {
    let mut buf = [0u8; 5];
    reader.read_exact(&mut buf)?;
    if buf != MAGIC {
        return Err(OmniError::InvalidMagic);
    }
    Ok(())
}

/// Bekannte primäre Block-Typen als Konstanten.
pub mod types {
    pub const DATA:        u64 = 1;
    pub const CHECKSUM:    u64 = 2;
    pub const COMPRESSION: u64 = 3;
    pub const ENCRYPTION:  u64 = 4;
    pub const META:        u64 = 5;
    pub const INDEX:       u64 = 6;
    pub const PADDING:     u64 = 7;
}

/// Bekannte Namespaces für Daten-Blöcke.
pub mod namespace {
    pub const STANDARD: u64 = 0;
    pub const GRAPHICS: u64 = 1;
    pub const AUDIO:    u64 = 2;
    // 100+ = Custom
}

/// Bekannte Standard-Typ-IDs (Namespace 0).
pub mod type_id {
    pub const UTF8_STRING: u64 = 1;
    pub const INT32:       u64 = 2;
    pub const ZSTD:        u64 = 3;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn block_type_roundtrip() {
        let bt = BlockType::new([1u64, 0, 42]).unwrap();
        let encoded = bt.encode();
        let (decoded, _) = BlockType::decode(&encoded).unwrap();
        assert_eq!(bt, decoded);
    }

    #[test]
    fn block_roundtrip() {
        let bt = BlockType::new([types::DATA, namespace::STANDARD, type_id::UTF8_STRING]).unwrap();
        let payload = b"Hallo Omni!".to_vec();
        let block = Block::new(bt, payload.clone());

        let mut buf = Vec::new();
        block.write(&mut buf).unwrap();

        let mut cur = Cursor::new(&buf);
        let decoded = Block::read(&mut cur).unwrap();

        assert_eq!(decoded.block_type.primary(), types::DATA);
        assert_eq!(decoded.payload, payload);
    }

    #[test]
    fn skip_works() {
        let bt = BlockType::new([types::DATA, 0, 1]).unwrap();
        let block = Block::new(bt, b"ignoriere mich".to_vec());

        let mut buf = Vec::new();
        block.write(&mut buf).unwrap();
        // Zweiter Block dahinter
        let bt2 = BlockType::new([types::META]).unwrap();
        let block2 = Block::new(bt2, b"ich bin wichtig".to_vec());
        block2.write(&mut buf).unwrap();

        let mut cur = Cursor::new(&buf);
        skip_block(&mut cur).unwrap(); // ersten Block überspringen
        let decoded = Block::read(&mut cur).unwrap(); // zweiten lesen
        assert_eq!(decoded.block_type.primary(), types::META);
        assert_eq!(decoded.payload, b"ich bin wichtig");
    }

    #[test]
    fn magic_roundtrip() {
        let mut buf = Vec::new();
        write_magic(&mut buf).unwrap();
        let mut cur = Cursor::new(&buf);
        check_magic(&mut cur).unwrap();
    }

    #[test]
    fn magic_invalid_rejected() {
        let buf = vec![0x04, 0x4F, 0x4D, 0x4E, 0x4A]; // OMNJ statt OMNI
        let mut cur = Cursor::new(&buf);
        assert!(check_magic(&mut cur).is_err());
    }
}