use crc32fast::Hasher;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Hash40(pub u64);

impl Hash40 {
    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn len(self) -> u8 {
        (self.0 >> 32) as u8
    }

    pub fn crc32(self) -> u32 {
        self.0 as u32
    }
}

impl From<&Hash40> for Hash40 {
    fn from(hash: &Hash40) -> Self {
        *hash
    }
}

impl From<u64> for Hash40 {
    fn from(hash: u64) -> Self {
        Hash40(hash)
    }
}

impl From<&str> for Hash40 {
    fn from(string: &str) -> Self {
        hash40(string)
    }
}

// Find the hash40 of a given string
pub fn hash40(string: &str) -> Hash40 {
    let bytes = string.as_bytes();

    Hash40(((bytes.len() as u64) << 32) + crc32(bytes) as u64)
}

fn crc32(bytes: &[u8]) -> u32 {
    let mut hasher = Hasher::new();
    hasher.update(bytes);
    hasher.finalize()
}
