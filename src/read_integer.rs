//! Provides the features for reading integers from binary data.

use std::io;
use endian::Endian;

/// Provides features for reading binary data.
///
/// # Examples
///
/// ```
///
/// use std::io;
/// use mm_binary_io::read_integer::ReadInteger;
/// use mm_binary_io::endian::{BigEndian};
///
/// let data = vec![
///     0x12,
///     0xFF, 0xFE,
///     0x12, 0x34, 0x56, 0x78,
///     0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFD];
///
/// let mut reader = io::Cursor::new(data);
///
/// let v_u8 = u8::read_integer::<BigEndian>(&mut reader).unwrap();
/// assert_eq!(0x12, v_u8);
///
/// let v_i16 = i16::read_integer::<BigEndian>(&mut reader).unwrap();
/// assert_eq!(-2, v_i16);
///
/// let v_u32 = u32::read_integer::<BigEndian>(&mut reader).unwrap();
/// assert_eq!(0x12345678, v_u32);
///
/// let v_i64 = i64::read_integer::<BigEndian>(&mut reader).unwrap();
/// assert_eq!(-3, v_i64);
///
/// ```
///
pub trait ReadInteger {
    /// The output integer type.
    type OutputType;

    /// Reads an integer from the reader.
    ///
    /// # Errors
    ///
    /// If the function succeeds, returns Ok(Self::OutputType),
    /// otherwise returns Err(io::Error)
    ///
    fn read_integer<TEndian>(reader: &mut io::Read) -> io::Result<Self::OutputType>
        where TEndian: Endian;
}

impl ReadInteger for u8 {
    type OutputType = u8;

    fn read_integer<TEndian>(reader: &mut io::Read) -> io::Result<Self::OutputType>
        where TEndian: Endian {
        let mut buf: &mut [u8] = &mut [0; 1];

        reader.read_exact(&mut buf)
            .and_then(|_| {
                Ok(TEndian::u8_from_bytes(&buf))
            })
    }
}

impl ReadInteger for i8 {
    type OutputType = i8;

    fn read_integer<TEndian>(reader: &mut io::Read) -> io::Result<Self::OutputType>
        where TEndian: Endian {
        u8::read_integer::<TEndian>(reader)
            .and_then(|x| {
                Ok(x as i8)
            })
    }
}


impl ReadInteger for u16 {
    type OutputType = u16;

    fn read_integer<TEndian>(reader: &mut io::Read) -> io::Result<Self::OutputType>
        where TEndian: Endian {
        let mut buf: &mut [u8] = &mut [0; 2];

        reader.read_exact(&mut buf)
            .and_then(|_| {
                Ok(TEndian::u16_from_bytes(&buf))
            })
    }
}

impl ReadInteger for i16 {
    type OutputType = i16;

    fn read_integer<TEndian>(reader: &mut io::Read) -> io::Result<Self::OutputType>
        where TEndian: Endian {
        u16::read_integer::<TEndian>(reader)
            .and_then(|x| {
                Ok(x as i16)
            })
    }
}

impl ReadInteger for u32 {
    type OutputType = u32;

    fn read_integer<TEndian>(reader: &mut io::Read) -> io::Result<Self::OutputType>
        where TEndian: Endian {
        let mut buf: &mut [u8] = &mut [0; 4];

        reader.read_exact(&mut buf)
            .and_then(|_| {
                Ok(TEndian::u32_from_bytes(&buf))
            })
    }
}

impl ReadInteger for i32 {
    type OutputType = i32;

    fn read_integer<TEndian>(reader: &mut io::Read) -> io::Result<Self::OutputType>
        where TEndian: Endian {
        u32::read_integer::<TEndian>(reader)
            .and_then(|x| {
                Ok(x as i32)
            })
    }
}

impl ReadInteger for u64 {
    type OutputType = u64;

    fn read_integer<TEndian>(reader: &mut io::Read) -> io::Result<Self::OutputType>
        where TEndian: Endian {
        let mut buf: &mut [u8] = &mut [0; 8];

        reader.read_exact(&mut buf)
            .and_then(|_| {
                Ok(TEndian::u64_from_bytes(&buf))
            })
    }
}

impl ReadInteger for i64 {
    type OutputType = i64;

    fn read_integer<TEndian>(reader: &mut io::Read) -> io::Result<Self::OutputType>
        where TEndian: Endian {
        u64::read_integer::<TEndian>(reader)
            .and_then(|x| {
                Ok(x as i64)
            })
    }
}
