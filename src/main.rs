pub mod block;
pub mod error;
pub mod varint;

use crate::block::{check_magic, namespace, type_id, types, write_magic, Block, BlockType};
use crate::varint::{decode_index_footer, write_index_footer};
use std::io::{Cursor, Seek, SeekFrom};

fn main() -> Result<(), crate::error::OmniError> {
    println!("=== OMNI Format Test ===\n");

    let mut buf = Cursor::new(Vec::<u8>::new());

    // ── Schreiben ────────────────────────────────────────────────────

    // Magic Number
    write_magic(&mut buf)?;
    println!("[+] Magic geschrieben  @ {}", pos(&mut buf));

    // Block 1: UTF-8 String
    let b1_start = buf.stream_position()?;
    let bt1 = BlockType::new([types::DATA, namespace::STANDARD, type_id::UTF8_STRING])?;
    Block::new(bt1, b"Hallo aus dem OMNI-Format!".to_vec()).write(&mut buf)?;
    println!("[+] Daten-Block        @ {b1_start} .. {}", pos(&mut buf));

    // Block 2: Meta
    let b2_start = buf.stream_position()?;
    let bt2 = BlockType::new([types::META])?;
    Block::new(bt2, b"version=1".to_vec()).write(&mut buf)?;
    println!("[+] Meta-Block         @ {b2_start} .. {}", pos(&mut buf));

    // Block 3: ein unbekannter Block (Typ 99) – soll später übersprungen werden
    let b3_start = buf.stream_position()?;
    let bt3 = BlockType::new([99u64])?;
    Block::new(bt3, b"unbekannte zukuenftige daten".to_vec()).write(&mut buf)?;
    println!("[+] Unbekannter Block  @ {b3_start} .. {}", pos(&mut buf));

    // Index-Block (zeigt auf Block 1)
    // Inhalt: einfach die absolute Position von Block 1 als Varint
    let index_start = buf.stream_position()?;
    let bt_idx = BlockType::new([types::INDEX])?;
    let index_payload = crate::varint::encode_varint(b1_start);
    Block::new(bt_idx, index_payload).write(&mut buf)?;
    println!("[+] Index-Block        @ {index_start} .. {}", pos(&mut buf));

    // Footer: [0x00][relativer Offset zum Index-Block-Anfang]
    let after_blocks = buf.stream_position()?;
    let relative_offset = after_blocks - index_start;
    write_index_footer(&mut buf, relative_offset)?;
    println!("[+] Footer geschrieben  (Offset={relative_offset})\n");

    let file_bytes = buf.get_ref().clone();
    println!("Dateigröße: {} Bytes\n", file_bytes.len());

    // ── Lesen ────────────────────────────────────────────────────────

    let mut r = Cursor::new(&file_bytes);

    // 1. Magic prüfen
    check_magic(&mut r)?;
    println!("[✓] Magic OK");

    // 2. Footer lesen → relativer Offset → absolute Position des Index-Blocks
    let relative = decode_index_footer(&file_bytes)?;
    let eof = file_bytes.len() as u64;
    // Footer liegt NACH den Blöcken: EOF - footer_size
    // footer_size = 1 (0x00) + varint_len(relative)
    let footer_varint = crate::varint::encode_varint(relative);
    let footer_size = 1 + footer_varint.len() as u64;
    let index_pos = eof - footer_size - relative;
    println!("[✓] Index-Block @ absolut={index_pos}");

    // 3. Index-Block lesen → enthält absolute Position von Block 1
    r.seek(SeekFrom::Start(index_pos))?;
    let index_block = Block::read(&mut r)?;
    let (b1_absolute, _) = crate::varint::decode_varint(&index_block.payload)?;
    println!("[✓] Index zeigt auf Block 1 @ {b1_absolute}");

    // 4. Direkt zu Block 1 springen (kein sequenzielles Lesen nötig)
    r.seek(SeekFrom::Start(b1_absolute))?;
    let block1 = Block::read(&mut r)?;
    println!("[✓] Block 1 Typ:    {}", block1.block_type.primary());
    println!("[✓] Block 1 Inhalt: {}", String::from_utf8_lossy(&block1.payload));

    // 5. Sequenziell lesen – unbekannten Block überspringen
    println!("\n--- Sequenzieller Scan ab Magic ---");
    r.seek(SeekFrom::Start(5))?; // nach Magic
    loop {
        let cur = r.stream_position()?;
        if cur >= index_pos { break; }  // Index-Block und Footer nicht mehr scannen

        // Peek: Header lesen ohne Payload zu laden
        let header = crate::block::read_header_skip_payload(&mut r)?;
        match header.block_type.primary() {
            types::DATA  => println!("  → Daten-Block   (bekannt, verarbeitet)"),
            types::META  => println!("  → Meta-Block    (bekannt, verarbeitet)"),
            unknown      => println!("  → Typ {unknown:3}        (unbekannt, übersprungen)"),
        }
    }

    println!("\n=== Alles OK ✅ ===");
    Ok(())
}

fn pos(c: &mut Cursor<Vec<u8>>) -> u64 {
    c.stream_position().unwrap()
}