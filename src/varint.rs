const CONTINUATION_BIT: u8 = 0x80;
const DATA_BITS_MASK: u8 = 0x7F;
const BITS_PER_VARINT_BYTE: usize = 7;

const fn varint_max_bytes(type_bits: usize) -> usize {
    (type_bits + BITS_PER_VARINT_BYTE - 1) / BITS_PER_VARINT_BYTE
}

// ── Traits (vor dem Macro!) ──────────────────────────────────────────────────

pub trait IntoVarint {
    fn to_varint(self) -> Vec<u8>;
}

pub trait FromVarint: Sized {
    fn from_varint(bytes: &[u8]) -> Option<(Self, usize)>;
    //                                      ^^^^^  ^^^^^^
    //                                      value  bytes consumed
}

// ── Macro ────────────────────────────────────────────────────────────────────

macro_rules! impl_varint {
    ($($t:ty),*) => {
        $(
            impl IntoVarint for $t {
                fn to_varint(self) -> Vec<u8> {
                    let mut buf = Vec::with_capacity(varint_max_bytes(<$t>::BITS as usize));
                    let mut val = self;
                    loop {
                        // Explizite Klammern: 'as' hat niedrigere Priorität als '&'
                        let byte = (val & (DATA_BITS_MASK as $t)) as u8;
                        val >>= BITS_PER_VARINT_BYTE;
                        if val != 0 {
                            buf.push(byte | CONTINUATION_BIT); // weitere Bytes folgen
                        } else {
                            buf.push(byte); // letztes Byte
                            break;
                        }
                    }
                    buf
                }
            }

            impl FromVarint for $t {
                fn from_varint(bytes: &[u8]) -> Option<(Self, usize)> {
                    let mut result: $t = 0;
                    let mut shift = 0usize;
                    let type_bits = <$t>::BITS as usize;

                    for (i, &byte) in bytes.iter().enumerate() {
                        let data = (byte & DATA_BITS_MASK) as $t;

                        // 1. Wenn wir schon so weit geschoben haben, dass kein Bit mehr
                        // vom Typ T aufgenommen werden kann:
                        if shift >= type_bits {
                            if data != 0 { return None; }
                        } else {
                            // 2. Wie viele Plätze sind "oben" noch frei?
                            let remaining = type_bits - shift;

                            // Wenn das neue Paket (7 Bit) größer ist als der Restplatz,
                            // schauen wir, ob in den "Überhang-Bits" etwas drinsteht.
                            if remaining < BITS_PER_VARINT_BYTE {
                                if (data >> remaining) != 0 {
                                    return None; // Überlauf!
                                }
                            }
                        }

                        // Der eigentliche Shift passiert nur, wenn wir noch im Rahmen sind.
                        // Das verhindert Panics bei shift >= BITS.
                        if shift < type_bits {
                            result |= data << shift;
                        }

                        shift += BITS_PER_VARINT_BYTE;

                        if byte & CONTINUATION_BIT == 0 {
                            return Some((result, i + 1));
                        }
                    }
                    None
                }
            }
        )*
    };
}

impl_varint!(u8, u16, u32, u64, u128, usize);