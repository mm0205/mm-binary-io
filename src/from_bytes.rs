use endian::Endian;

pub trait FromBytes {
    type OutputType;

    fn from_bytes<TEndian>(bytes: &[u8]) -> Self::OutputType
        where TEndian: Endian;
}

impl FromBytes for u8 {
    type OutputType = u8;

    fn from_bytes<TEndian>(bytes: &[u8]) -> Self::OutputType
        where TEndian: Endian {
        TEndian::u8_from_bytes(bytes)
    }
}

impl FromBytes for i8 {
    type OutputType = i8;

    fn from_bytes<TEndian>(bytes: &[u8]) -> Self::OutputType
        where TEndian: Endian {
        TEndian::i8_from_bytes(bytes)
    }
}

impl FromBytes for u16 {
    type OutputType = u16;

    fn from_bytes<TEndian>(bytes: &[u8]) -> Self::OutputType
        where TEndian: Endian {
        TEndian::u16_from_bytes(bytes)
    }
}

impl FromBytes for i16 {
    type OutputType = i16;

    fn from_bytes<TEndian>(bytes: &[u8]) -> Self::OutputType
        where TEndian: Endian {
        TEndian::i16_from_bytes(bytes)
    }
}

impl FromBytes for u32 {
    type OutputType = u32;

    fn from_bytes<TEndian>(bytes: &[u8]) -> Self::OutputType
        where TEndian: Endian {
        TEndian::u32_from_bytes(bytes)
    }
}

impl FromBytes for i32 {
    type OutputType = i32;

    fn from_bytes<TEndian>(bytes: &[u8]) -> Self::OutputType
        where TEndian: Endian {
        TEndian::i32_from_bytes(bytes)
    }
}

impl FromBytes for u64 {
    type OutputType = u64;

    fn from_bytes<TEndian>(bytes: &[u8]) -> Self::OutputType
        where TEndian: Endian {
        TEndian::u64_from_bytes(bytes)
    }
}

impl FromBytes for i64 {
    type OutputType = i64;

    fn from_bytes<TEndian>(bytes: &[u8]) -> Self::OutputType
        where TEndian: Endian {
        TEndian::i64_from_bytes(bytes)
    }
}

