use std::str;
use std::convert::From;

#[derive(Clone, Copy)]
pub enum ByteStr {
    Byte8([u8; 8]),
    Byte16([u8; 16]),
    Byte32([u8; 32]),
    Byte64([u8; 64]),
}

impl Default for ByteStr {
    fn default() -> Self {
        ByteStr::Byte8([0; 8])
    }
}

impl PartialEq<ByteStr> for ByteStr {
    fn eq(&self, other: &ByteStr) -> bool {
        match (*self, *other) {
            (ByteStr::Byte8(s), ByteStr::Byte8(o)) => (0..8).into_iter().all(|i| s[i] == o[i]),
            (ByteStr::Byte16(s), ByteStr::Byte16(o)) => (0..16).into_iter().all(|i| s[i] == o[i]),
            (ByteStr::Byte32(s), ByteStr::Byte32(o)) => (0..32).into_iter().all(|i| s[i] == o[i]),
            (ByteStr::Byte64(s), ByteStr::Byte64(o)) => (0..64).into_iter().all(|i| s[i] == o[i]),
            _ => false
        }
    }
}

impl PartialEq<&str> for ByteStr {
    fn eq(&self, other: &&str) -> bool {
        let bytes = other.as_bytes();
        match *self {
            ByteStr::Byte8(s) => bytes.len() <= 8 && (0..bytes.len()).into_iter().all(|i| s[i] == bytes[i]),
            ByteStr::Byte16(s) => bytes.len() <= 16 && (0..bytes.len()).into_iter().all(|i| s[i] == bytes[i]),
            ByteStr::Byte32(s) => bytes.len() <= 32 && (0..bytes.len()).into_iter().all(|i| s[i] == bytes[i]),
            ByteStr::Byte64(s) => bytes.len() <= 64 && (0..bytes.len()).into_iter().all(|i| s[i] == bytes[i]),
        }
    }
}

impl From<&str> for ByteStr {
    fn from(s: &str) -> ByteStr {
        let bytes = s.as_bytes();

        match bytes {
            b if b.len() <= 8 => { 
                let mut array = [0; 8];
                for i in 0..bytes.len() { array[i] = b[i]; } 
                ByteStr::Byte8(array) 
            },
            b if b.len() <= 16 => { 
                let mut array = [0; 16];
                for i in 0..bytes.len() { array[i] = b[i]; } 
                ByteStr::Byte16(array) 
            },
            b if b.len() <= 32 => { 
                let mut array = [0; 32];
                for i in 0..bytes.len() { array[i] = b[i]; } 
                ByteStr::Byte32(array) 
            },
            _ => { 
                let mut array = [0; 64];
                for i in 0..bytes.len() { 
                    if i < 64 { array[i] = bytes[i]; } 
                    else { break; }
                }
                ByteStr::Byte64(array) 
            },          
        }
    }
}

impl Into<String> for ByteStr {
    fn into(self) -> String {
        let c:Vec<u8> = match self {
            ByteStr::Byte8(b) => b.iter().map(|x| *x).collect(),
            ByteStr::Byte16(b) => b.iter().map(|x| *x).collect(),
            ByteStr::Byte32(b) => b.iter().map(|x| *x).collect(),
            ByteStr::Byte64(b) => b.iter().map(|x| *x).collect(),
        };
        String::from_utf8(c).unwrap()
    }
}
