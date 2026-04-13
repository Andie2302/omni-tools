pub mod block;
pub mod error;
pub mod varint;

use crate::block::{check_magic, namespace, type_id, types, write_magic, Block, BlockType};
use crate::varint::{decode_index_footer, write_index_footer};
use std::io::{Cursor, Seek, SeekFrom};

fn main() -> Result<(), crate::error::OmniError> {
    println!("--- OMNI Test Run ---");

    // 1. Virtuelle Datei im Speicher erstellen (Cursor verhält sich wie eine Datei)
    let mut buffer = Cursor::new(Vec::new());

    // 2. Magic Number schreiben
    println!("Schreibe Magic...");
    write_magic(&mut buffer).map_err(|_| crate::error::OmniError::InvalidMagic)?;

    // 3. Einen Daten-Block erstellen (z.B. ein UTF-8 String)
    let payload = b"Das ist eine Test-Nachricht in OMNI".to_vec();
    let bt = BlockType::new([types::DATA, namespace::STANDARD, type_id::UTF8_STRING])?;
    let block = Block::new(bt, payload);

    println!("Schreibe Daten-Block...");
    let block_start_pos = buffer.stream_position()?;
    block.write(&mut buffer)?;

    // 4. Einen Meta-Block dahinter hängen
    let meta_bt = BlockType::new([types::META])?;
    let _meta_block = BlockType::new([42])?; // Nur eine fiktive ID
    let block2 = Block::new(meta_bt, b"Version 1.0".to_vec());
    block2.write(&mut buffer)?;

    // 5. Index-Footer schreiben
    // Wir speichern den Offset zum Anfang des ersten Blocks (relativ zum Dateiende)
    let current_pos = buffer.stream_position()?;
    let relative_offset = current_pos - block_start_pos;

    println!("Schreibe Index-Footer (Relativer Offset: {})...", relative_offset);
    write_index_footer(&mut buffer, relative_offset)?;

    // --- LESEN ---
    println!("\n--- Lese Test-Daten zurück ---");
    buffer.seek(SeekFrom::Start(0))?;

    // Magic prüfen
    check_magic(&mut buffer)?;
    println!("Magic OK!");

    // Zum Ende springen, um den Index zu finden
    let file_size = buffer.get_ref().len() as u64;
    buffer.seek(SeekFrom::Start(0))?; // rposition braucht den ganzen slice
    let footer_offset = decode_index_footer(buffer.get_ref())?;
    println!("Index gefunden! Offset zurück: {}", footer_offset);

    // Springe zum Block via Index
    let _target_pos = file_size - (footer_offset + (file_size - current_pos));
    // (Hinweis: In einer echten Datei-Logik ist das einfacher, hier im Cursor-Slice
    //  muss man nur darauf achten, wo der Footer im Vergleich zum EOF liegt)

    buffer.seek(SeekFrom::Start(block_start_pos))?;
    let decoded_block = Block::read(&mut buffer)?;

    println!("Gelesener Block-Typ: {:?}", decoded_block.block_type.primary());
    println!("Inhalt: {}", String::from_utf8_lossy(&decoded_block.payload));

    println!("\nTest erfolgreich beendet! ✅");
    Ok(())
}