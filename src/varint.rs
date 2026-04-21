use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::str::FromStr;

// ============================================================================
// Constants
// ============================================================================

const CONTINUATION_BIT: u8 = 0x80;
const DATA_BITS_MASK: u8 = 0x7F;
const BITS_PER_VARINT_BYTE: usize = 7;

const fn varint_max_bytes(type_bits: usize) -> usize {
    (type_bits + BITS_PER_VARINT_BYTE - 1) / BITS_PER_VARINT_BYTE
}

// ============================================================================
// Traits
// ============================================================================

pub trait IntoVarint {
    fn to_varint(self) -> Vec<u8>;
}

pub trait FromVarint: Sized {
    fn from_varint(bytes: &[u8]) -> Option<(Self, usize)>;
}

// ============================================================================
// VarInt – immer in einem gültigen Zustand
// ============================================================================

/// Ein VarInt ist garantiert gültig: Der private Konstruktor stellt sicher,
/// dass `data` stets ein korrekt kodiertes VarInt enthält.
/// Ungültige Bytes können von außen nicht eingeschleust werden.
#[derive(Clone)]
pub struct VarInt {
    data: Vec<u8>, // privat: nur über From/TryFrom/FromStr konstruierbar
}

impl VarInt {
    /// Interner Konstruktor – nur innerhalb dieses Moduls nutzbar.
    fn from_raw(data: Vec<u8>) -> Self {
        VarInt { data }
    }

    pub fn try_convert<T: FromVarint>(&self) -> Option<T> {
        T::from_varint(&self.data).map(|(val, _)| val)
    }

    pub fn fits_in<T: FromVarint>(&self) -> bool {
        self.try_convert::<T>().is_some()
    }

    /// Dekodiert zu u128 – immer erfolgreich, da VarInt immer gültig ist.
    fn as_u128(&self) -> u128 {
        // SAFETY: `data` wurde ausschließlich über kodierte Werte erzeugt.
        u128::from_varint(&self.data)
            .expect("VarInt invariant violated: data is always valid")
            .0
    }
}

// ============================================================================
// Macro: IntoVarint, FromVarint, From, TryFrom für Integer-Typen
// ============================================================================

macro_rules! impl_varint {
    ($($t:ty),*) => {
        $(
            impl IntoVarint for $t {
                fn to_varint(self) -> Vec<u8> {
                    let mut buf = Vec::with_capacity(varint_max_bytes(<$t>::BITS as usize));
                    let mut val = self;
                    loop {
                        let byte = (val & DATA_BITS_MASK as $t) as u8;
                        val >>= BITS_PER_VARINT_BYTE;
                        if val != 0 {
                            buf.push(byte | CONTINUATION_BIT);
                        } else {
                            buf.push(byte);
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
                        if shift >= type_bits {
                            if data != 0 { return None; }
                        } else {
                            let remaining = type_bits - shift;
                            if remaining < BITS_PER_VARINT_BYTE && (data >> remaining) != 0 {
                                return None;
                            }
                        }
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

            /// Infallible: Jeder Integer-Wert ist ein gültiges VarInt.
            impl From<$t> for VarInt {
                fn from(val: $t) -> Self {
                    VarInt::from_raw(val.to_varint())
                }
            }

            /// Fallible: Ein VarInt passt nicht in jeden kleineren Typ.
            impl TryFrom<VarInt> for $t {
                type Error = &'static str;
                fn try_from(vi: VarInt) -> Result<Self, Self::Error> {
                    vi.try_convert::<$t>().ok_or("VarInt value overflows target type")
                }
            }

            impl TryFrom<&VarInt> for $t {
                type Error = &'static str;
                fn try_from(vi: &VarInt) -> Result<Self, Self::Error> {
                    vi.try_convert::<$t>().ok_or("VarInt value overflows target type")
                }
            }
        )*
    };
}

impl_varint!(u8, u16, u32, u64, u128, usize);

// ============================================================================
// Standard-Trait-Implementierungen
// ============================================================================

impl fmt::Display for VarInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_u128())
    }
}

impl fmt::Debug for VarInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VarInt({} | {:?})", self.as_u128(), self.data)
    }
}

impl Deref for VarInt {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.data
    }
}

impl AsRef<[u8]> for VarInt {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}

// Eq über den dekodierten Wert – nicht über die Bytes,
// da nicht-kanonische Kodierungen denselben Wert repräsentieren könnten.
impl PartialEq for VarInt {
    fn eq(&self, other: &Self) -> bool {
        self.as_u128() == other.as_u128()
    }
}

impl Eq for VarInt {}

// Hash muss konsistent mit PartialEq sein: gleicher Wert → gleicher Hash.
impl Hash for VarInt {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_u128().hash(state);
    }
}

impl PartialOrd for VarInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VarInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_u128().cmp(&other.as_u128())
    }
}

// FromStr parst über u128 – deckt den vollen darstellbaren Wertebereich ab.
impl FromStr for VarInt {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u128>()
            .map(VarInt::from)
            .map_err(|_| "Not a valid decimal number")
    }
}

impl<'a> IntoIterator for &'a VarInt {
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}