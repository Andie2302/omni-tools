mod varint;
use varint::{IntoVarint, FromVarint};

fn main() {
    println!("🚀 Starte Varint Tests...");
    println!("---------------------------------------");

    // Test 1: Einfache Zahlen (u32)
    let test_u32 = 300u32;
    let encoded = test_u32.to_varint();
    println!("Test u32 (300): Bytes: {:02X?}, Länge: {}", encoded, encoded.len());

    if let Some((decoded, read)) = u32::from_varint(&encoded) {
        println!("✅ Dekodiert: {}, Bytes gelesen: {}", decoded, read);
        assert_eq!(test_u32, decoded);
    }

    println!("---------------------------------------");

    // Test 2: usize
    let test_usize: usize = 12345678;
    let encoded_usize = test_usize.to_varint();
    let (decoded_usize, _) = usize::from_varint(&encoded_usize).unwrap();
    println!("Test usize ({}): Bytes: {:02X?}", test_usize, encoded_usize);
    assert_eq!(test_usize, decoded_usize);
    println!("✅ usize Test erfolgreich");

    println!("---------------------------------------");

    // Test 3: u8 Mauer
    println!("Test u8 Mauer (300 in u8 lesen):");
    let result_u8 = u8::from_varint(&encoded);
    match result_u8 {
        Some((val, _)) => println!("❌ Fehler: Konnte 300 in u8 lesen ({})", val),
        None => println!("✅ Korrekt: 300 passt nicht in u8, None zurückgegeben."),
    }

    println!("---------------------------------------");

    // Test 4: u128
    let riesig: u128 = u128::MAX;
    let encoded_riesig = riesig.to_varint();
    println!("Test u128 MAX: Varint-Länge: {} Bytes", encoded_riesig.len());
    let (decoded_riesig, _) = u128::from_varint(&encoded_riesig).unwrap();
    assert_eq!(riesig, decoded_riesig);
    println!("✅ u128 Test erfolgreich");

    println!("---------------------------------------");
    println!("🎉 Alle Tests bestanden!");
}