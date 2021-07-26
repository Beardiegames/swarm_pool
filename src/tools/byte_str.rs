//! Sized byte string with predefined sizes of 8, 16, 32 or 64 characters/bytes
//! 
//! Examples
//! ```
//! use swarm_pool::tools::byte_str::ByteStr;
//! 
//! let byte_str: ByteStr = ByteStr::from("test");
//! assert_eq!(byte_str, "test");
//! ```

use std::str;
use std::convert::From;
use std::fmt;
//use std::fmt::Debug;

/// Sized byte string with predefined sizes of 8, 16, 32 or 64 characters/bytes
/// 
/// Examples
/// ```
/// use swarm_pool::tools::byte_str::ByteStr;
/// 
/// let byte_str: ByteStr = ByteStr::from("test");
/// assert_eq!(byte_str, "test");
/// ```
#[derive(Clone, Copy)]
pub enum ByteStr {
    Str8([u8; 8]),
    Str16([u8; 16]),
    Str32([u8; 32]),
    Str64([u8; 64]),
}

impl ByteStr {
    /// returns the bytesize of the current ByteStr
    /// 
    /// Examples
    /// ```
    /// use swarm_pool::tools::byte_str::ByteStr;
    /// 
    /// let byte_str: ByteStr = ByteStr::from("test");
    /// assert_eq!(byte_str, "test");
    /// 
    /// // ByteStr::from automatically picks an apporiate byte length
    /// // the remaining unused characters are filled with zero's
    /// let str_len = byte_str.len(); 
    /// assert_eq!(str_len, 8); 
    /// ```
    pub fn len(&self) -> usize {
        match self {
            ByteStr::Str8(s) => s.len(),
            ByteStr::Str16(s) => s.len(),
            ByteStr::Str32(s) => s.len(),
            ByteStr::Str64(s) => s.len(),
        }
    }

    /// Returns the character at a specific index as a u8 byte
    /// 
    /// Examples
    /// ```
    /// use swarm_pool::tools::byte_str::ByteStr;
    /// 
    /// let byte_str: ByteStr = ByteStr::from("test");
    /// assert_eq!(byte_str, "test");
    /// 
    /// // ByteStr::from automatically picks an apporiate byte length
    /// // the remaining unused characters are filled with zero's
    /// assert_eq!(byte_str.char(0), "t".as_bytes()[0]);
    /// assert_eq!(byte_str.char(1), "e".as_bytes()[0]);
    /// assert_eq!(byte_str.char(2), "s".as_bytes()[0]);
    /// assert_eq!(byte_str.char(3), "t".as_bytes()[0]);
    /// assert_eq!(byte_str.char(4), 0);
    /// assert_eq!(byte_str.char(5), 0);
    /// assert_eq!(byte_str.char(6), 0);
    /// assert_eq!(byte_str.char(7), 0);
    /// ```
    pub fn char(&self, at_index: usize) -> u8 {
        match self {
            ByteStr::Str8(s) => s[at_index],
            ByteStr::Str16(s) => s[at_index],
            ByteStr::Str32(s) => s[at_index],
            ByteStr::Str64(s) => s[at_index],
        }
    }
}

/// Default ByteStr is ByteStr::Str8([0; 8])
impl Default for ByteStr {
    fn default() -> Self {
        ByteStr::Str8([0; 8])
    }
}

/// Debug formatting for ByteStr
impl fmt::Debug for ByteStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value: String = self.clone().into();
        write!(f, "\"{}\"", value)
    }
}

/// Compare a ByteStr with another ByteStr
/// 
/// Examples
/// ```
/// use swarm_pool::tools::byte_str::ByteStr;
/// 
/// let byte_str_1: ByteStr = ByteStr::from("test");
/// let byte_str_2: ByteStr = ByteStr::from("test");
/// assert_eq!(byte_str_1, byte_str_2);
/// assert!(byte_str_1 == byte_str_2);
impl PartialEq<ByteStr> for ByteStr {
    fn eq(&self, other: &ByteStr) -> bool {
        match (*self, *other) {
            (ByteStr::Str8(s), ByteStr::Str8(o)) => (0..8).into_iter().all(|i| s[i] == o[i]),
            (ByteStr::Str16(s), ByteStr::Str16(o)) => (0..16).into_iter().all(|i| s[i] == o[i]),
            (ByteStr::Str32(s), ByteStr::Str32(o)) => (0..32).into_iter().all(|i| s[i] == o[i]),
            (ByteStr::Str64(s), ByteStr::Str64(o)) => (0..64).into_iter().all(|i| s[i] == o[i]),
            _ => false
        }
    }
}

/// Compare a ByteStr with a &str
/// 
/// Examples
/// ```
/// use swarm_pool::tools::byte_str::ByteStr;
/// 
/// let byte_str: ByteStr = ByteStr::from("test");
/// assert_eq!(byte_str, "test");
/// assert!(byte_str == "test");
impl PartialEq<&str> for ByteStr {
    fn eq(&self, other: &&str) -> bool {
        let bytes = other.as_bytes();
        match *self {
            ByteStr::Str8(s) => bytes.len() <= 8 && (0..bytes.len()).into_iter().all(|i| s[i] == bytes[i]),
            ByteStr::Str16(s) => bytes.len() <= 16 && (0..bytes.len()).into_iter().all(|i| s[i] == bytes[i]),
            ByteStr::Str32(s) => bytes.len() <= 32 && (0..bytes.len()).into_iter().all(|i| s[i] == bytes[i]),
            ByteStr::Str64(s) => bytes.len() <= 64 && (0..bytes.len()).into_iter().all(|i| s[i] == bytes[i]),
        }
    }
}

/// Compare a ByteStr with a String
/// 
/// Examples
/// ```
/// use swarm_pool::tools::byte_str::ByteStr;
/// 
/// let byte_str: String = ByteStr::from("test").into();
/// assert_eq!(byte_str, String::from("test"));
/// assert!(byte_str == String::from("test"));
impl PartialEq<String> for ByteStr {
    fn eq(&self, other: &String) -> bool {
        let bytes = other.as_bytes();
        match *self {
            ByteStr::Str8(s) => bytes.len() <= 8 && (0..bytes.len()).into_iter().all(|i| s[i] == bytes[i]),
            ByteStr::Str16(s) => bytes.len() <= 16 && (0..bytes.len()).into_iter().all(|i| s[i] == bytes[i]),
            ByteStr::Str32(s) => bytes.len() <= 32 && (0..bytes.len()).into_iter().all(|i| s[i] == bytes[i]),
            ByteStr::Str64(s) => bytes.len() <= 64 && (0..bytes.len()).into_iter().all(|i| s[i] == bytes[i]),
        }
    }
}

/// Convert a &str into a ByteStr
/// 
/// Examples
/// ```
/// use swarm_pool::tools::byte_str::ByteStr;
/// 
/// let byte_str: ByteStr = ByteStr::from("test");
/// assert_eq!(byte_str, "test");
impl From<&str> for ByteStr {
    fn from(s: &str) -> ByteStr {
        let bytes = s.as_bytes();

        match bytes {
            b if b.len() <= 8 => { 
                let mut array = [0; 8];
                array[..bytes.len()].clone_from_slice(&b[..bytes.len()]);
                ByteStr::Str8(array) 
            },
            b if b.len() <= 16 => { 
                let mut array = [0; 16];
                array[..bytes.len()].clone_from_slice(&b[..bytes.len()]);
                ByteStr::Str16(array) 
            },
            b if b.len() <= 32 => { 
                let mut array = [0; 32];
                array[..bytes.len()].clone_from_slice(&b[..bytes.len()]);
                ByteStr::Str32(array) 
            },
            _ => { 
                let mut array = [0; 64];
                array[..bytes.len()].clone_from_slice(&bytes[..bytes.len()]);
                ByteStr::Str64(array) 
            },          
        }
    }
}

/// Convert a String into a ByteStr
/// 
/// Examples
/// ```
/// use swarm_pool::tools::byte_str::ByteStr;
/// 
/// let byte_str: ByteStr = ByteStr::from(String::from("test"));
/// assert_eq!(byte_str, "test");
impl From<String> for ByteStr {
    fn from(s: String) -> ByteStr {
        let bytes = s.as_bytes();

        match bytes {
            b if b.len() <= 8 => { 
                let mut array = [0; 8];
                for i in 0..bytes.len() { array[i] = b[i]; } 
                ByteStr::Str8(array) 
            },
            b if b.len() <= 16 => { 
                let mut array = [0; 16];
                for i in 0..bytes.len() { array[i] = b[i]; } 
                ByteStr::Str16(array) 
            },
            b if b.len() <= 32 => { 
                let mut array = [0; 32];
                for i in 0..bytes.len() { array[i] = b[i]; } 
                ByteStr::Str32(array) 
            },
            _ => { 
                let mut array = [0; 64];
                for i in 0..bytes.len() { 
                    if i < 64 { array[i] = bytes[i]; } 
                    else { break; }
                }
                ByteStr::Str64(array) 
            },          
        }
    }
}

/// Convert a ByteStr into a String
/// 
/// Examples
/// ```
/// use swarm_pool::tools::byte_str::ByteStr;
/// 
/// let byte_str: ByteStr = ByteStr::from("test");
/// let into_str: String = byte_str.into();
/// assert_eq!(into_str, String::from("test"));
impl Into<String> for ByteStr {
    fn into(self) -> String {
        let c:Vec<u8> = match self {
            ByteStr::Str8(b) => b.iter().take_while(|x| **x != 0).map(|x| *x).collect(),
            ByteStr::Str16(b) => b.iter().take_while(|x| **x != 0).map(|x| *x).collect(),
            ByteStr::Str32(b) => b.iter().take_while(|x| **x != 0).map(|x| *x).collect(),
            ByteStr::Str64(b) => b.iter().take_while(|x| **x != 0).map(|x| *x).collect(),
        };
        String::from_utf8(c).unwrap()
    }
}