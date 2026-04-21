use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

pub struct VarInt {
    pub data: Vec<u8>,
}

impl VarInt {
    pub fn new<T: IntoVarint>(val: T) -> Self {
        VarInt { data: val.to_varint() }
    }
    pub fn is_valid(&self) -> bool {
        u64::from_varint(&self.data).is_some()
    }
}


const CONTINUATION_BIT: u8 = 0x80;
const DATA_BITS_MASK: u8 = 0x7F;
const BITS_PER_VARINT_BYTE: usize = 7;
const fn varint_max_bytes(type_bits: usize) -> usize {
    (type_bits + BITS_PER_VARINT_BYTE - 1) / BITS_PER_VARINT_BYTE
}

pub trait IntoVarint {
    fn to_varint(self) -> Vec<u8>;
}

pub trait FromVarint: Sized {
    fn from_varint(bytes: &[u8]) -> Option<(Self, usize)>;
}

macro_rules! impl_varint {
    ($($t:ty),*) => {
        $(
            impl IntoVarint for $t {
                fn to_varint(self) -> Vec<u8> {
                    let mut buf = Vec::with_capacity(varint_max_bytes(<$t>::BITS as usize));
                    let mut val = self;
                    loop {
                        let byte = (val & (DATA_BITS_MASK as $t)) as u8;
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
                            if remaining < BITS_PER_VARINT_BYTE {
                                if (data >> remaining) != 0 {
                                    return None;
                                }
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
            impl From<$t> for VarInt {
                fn from(val: $t) -> Self {
                    VarInt { data: val.to_varint() }
                }
            }
            impl TryFrom<VarInt> for $t {
                type Error = &'static str;
                fn try_from(vi: VarInt) -> Result<Self, Self::Error> {
                    <$t>::from_varint(&vi.data)
                        .map(|(val, _)| val)
                        .ok_or("Ungültiger oder übergelaufener VarInt")
                }
            }
            impl TryFrom<&VarInt> for $t {
                type Error = &'static str;
                fn try_from(vi: &VarInt) -> Result<Self, Self::Error> {
                    <$t>::from_varint(&vi.data)
                        .map(|(val, _)| val)
                        .ok_or("Ungültiger oder übergelaufener VarInt")
                }
            }
        )*
    };
}

impl_varint!(u8, u16, u32, u64, u128, usize);
impl fmt::Display for VarInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((val, _)) = u64::from_varint(&self.data) {
            write!(f, "{}", val)
        } else {
            write!(f, "<ungültiger VarInt {:?}>", self.data)
        }
    }
}

impl fmt::Debug for VarInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VarInt({:?})", self.data)
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
impl PartialEq for VarInt {
    fn eq(&self, other: &Self) -> bool {
        u64::from_varint(&self.data).map(|(v, _)| v)
            == u64::from_varint(&other.data).map(|(v, _)| v)
    }
}

impl Eq for VarInt {}

impl PartialOrd for VarInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VarInt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = u64::from_varint(&self.data).map(|(v, _)| v).unwrap_or(0);
        let b = u64::from_varint(&other.data).map(|(v, _)| v).unwrap_or(0);
        a.cmp(&b)
    }
}
impl FromStr for VarInt {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: u64 = s.parse().map_err(|_| "Keine gültige Dezimalzahl")?;
        Ok(VarInt::from(n))
    }
}
impl<'a> IntoIterator for &'a VarInt {
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_u32() {
        let vi = VarInt::from(300u32);
        let back: u32 = (&vi).try_into().unwrap();
        assert_eq!(back, 300);
    }

    #[test]
    fn display() {
        assert_eq!(VarInt::from(42u64).to_string(), "42");
    }

    #[test]
    fn from_str() {
        let vi: VarInt = "1234".parse().unwrap();
        let val: u64 = (&vi).try_into().unwrap();
        assert_eq!(val, 1234);
    }

    #[test]
    fn ordering() {
        let a = VarInt::from(10u32);
        let b = VarInt::from(20u32);
        assert!(a < b);
    }

    #[test]
    fn deref_and_iter() {
        let vi = VarInt::from(1u8);
        assert_eq!(vi.len(), 1);
        assert_eq!(vi[0], 0x01);
        let bytes: Vec<u8> = (&vi).into_iter().copied().collect();
        assert_eq!(bytes, vec![0x01]);
    }

    #[test]
    fn canonical_equality() {
        let a = VarInt { data: vec![0x00] };
        let b = VarInt { data: vec![0x80, 0x00] };
        assert_eq!(a, b);
    }
}
