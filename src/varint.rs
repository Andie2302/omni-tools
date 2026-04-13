use crate::error::OmniError;
use std::io::{Read, Write};

/// Schreibt einen u64 als LEB128-Varint.
/// Gibt die Anzahl geschriebener Bytes zurück.
pub fn write_varint<W: Write>(writer: &mut W, mut val: u64) -> Result<usize, OmniError> {
    let mut count = 0;
    loop {
        let byte = (val & 0x7f) as u8;
        val >>= 7;
        if val == 0 {
            writer.write_all(&[byte])?;
            count += 1;
            break;
        } else {
            writer.write_all(&[byte | 0x80])?;
            count += 1;
        }
    }
    Ok(count)
}

/// Liest ein LEB128-Varint.
/// Gibt (wert, bytes_gelesen) zurück.
pub fn read_varint<R: Read>(reader: &mut R) -> Result<(u64, usize), OmniError> {
    let mut result: u64 = 0;
    let mut shift = 0u32;
    let mut count = 0;
    loop {
        let mut buf = [0u8; 1];
        reader.read_exact(&mut buf)?;
        count += 1;
        let byte = buf[0];
        if shift >= 63 && byte > 1 {
            return Err(OmniError::VarintOverflow);
        }
        result |= ((byte & 0x7f) as u64) << shift;
        shift += 7;
        if byte & 0x80 == 0 {
            return Ok((result, count));
        }
        if shift >= 70 {
            return Err(OmniError::VarintOverflow);
        }
    }
}

/// Kodiert einen u64 als LEB128 in einen Vec<u8>.
/// Nützlich wenn man die Bytes braucht ohne einen Writer.
pub fn encode_varint(mut val: u64) -> Vec<u8> {
    let mut out = Vec::with_capacity(10);
    loop {
        let byte = (val & 0x7f) as u8;
        val >>= 7;
        if val == 0 {
            out.push(byte);
            break;
        } else {
            out.push(byte | 0x80);
        }
    }
    out
}

/// Dekodiert ein LEB128-Varint aus einem Byte-Slice.
/// Gibt (wert, bytes_verbraucht) zurück.
pub fn decode_varint(bytes: &[u8]) -> Result<(u64, usize), OmniError> {
    let mut result: u64 = 0;
    let mut shift = 0u32;
    for (i, &byte) in bytes.iter().enumerate() {
        if shift >= 63 && byte > 1 {
            return Err(OmniError::VarintOverflow);
        }
        result |= ((byte & 0x7f) as u64) << shift;
        shift += 7;
        if byte & 0x80 == 0 {
            return Ok((result, i + 1));
        }
        if shift >= 70 {
            return Err(OmniError::VarintOverflow);
        }
    }
    Err(OmniError::UnexpectedEof)
}

/// Liest das End-of-File Sentinel und den Index-Offset.
///
/// Datei-Layout am Ende:
///   ... [Index-Block] [0x00] [LEB128: relativer Offset] EOF
///
/// Liest rückwärts: findet 0x00, dekodiert dann den Varint
/// der die Distanz vom 0x00 zurück zum Anfang des Index-Blocks angibt.
pub fn decode_index_footer(tail: &[u8]) -> Result<u64, OmniError> {
    // Suche 0x00 von rechts
    // Die Varint-Bytes stehen NACH dem 0x00 (Richtung EOF)
    // Layout: [0x00] [b0] [b1] ... [bN] EOF
    //         ^sentinel  ^--- LEB128 Offset ---^

    let sentinel_pos = tail
        .iter()
        .rposition(|&b| b == 0x00)
        .ok_or(OmniError::NoIndexFound)?;

    let varint_bytes = &tail[sentinel_pos + 1..];
    if varint_bytes.is_empty() {
        return Err(OmniError::NoIndexFound);
    }

    let (offset, _) = decode_varint(varint_bytes)?;
    if offset == 0 {
        return Err(OmniError::NoIndexFound);
    }
    Ok(offset)
}

/// Schreibt das End-of-File Sentinel + relativen Offset zum Index-Block.
pub fn write_index_footer<W: Write>(
    writer: &mut W,
    relative_offset: u64,
) -> Result<(), OmniError> {
    if relative_offset == 0 {
        return Err(OmniError::InvalidOffset);
    }
    writer.write_all(&[0x00])?;
    write_varint(writer, relative_offset)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn roundtrip_small() {
        for val in [0u64, 1, 127, 128, 255, 300, 16383, 16384, u64::MAX / 2] {
            let encoded = encode_varint(val);
            let (decoded, _) = decode_varint(&encoded).unwrap();
            assert_eq!(val, decoded, "Roundtrip fehlgeschlagen für {val}");
        }
    }

    #[test]
    fn roundtrip_via_writer() {
        let mut buf = Vec::new();
        write_varint(&mut buf, 300).unwrap();
        let mut cur = Cursor::new(&buf);
        let (val, _) = read_varint(&mut cur).unwrap();
        assert_eq!(val, 300);
    }

    #[test]
    fn footer_roundtrip() {
        let mut buf = Vec::new();
        write_index_footer(&mut buf, 12345).unwrap();
        let offset = decode_index_footer(&buf).unwrap();
        assert_eq!(offset, 12345);
    }

    #[test]
    fn footer_zero_rejected() {
        let result = write_index_footer(&mut Vec::new(), 0);
        assert!(result.is_err());
    }

    #[test]
    fn known_leb128_values() {
        // 300 = 0xAC 0x02 in LEB128
        assert_eq!(encode_varint(300), vec![0xAC, 0x02]);
        // 624485 = 0xE5 0x8E 0x26
        assert_eq!(encode_varint(624485), vec![0xE5, 0x8E, 0x26]);
    }
}