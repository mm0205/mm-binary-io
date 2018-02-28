//! Provides the features to write integers as binary data.

use std::io;
use endian::Endian;

/// Provides the features to write integer as binary data.
///
/// # Examples
///
/// ```
///
/// use std::io;
/// use mm_binary_io::endian::BigEndian;
/// use mm_binary_io::write_integer::WriteInteger;
///
/// let mut writer = io::Cursor::new(vec![]);
/// 0x12_u8.write_integer::<BigEndian>(&mut writer).unwrap();
/// assert_eq!(0x12, writer.into_inner()[0]);
///
/// let mut writer = io::Cursor::new(vec![]);
/// (-1 as i8) .write_integer::<BigEndian>(&mut writer).unwrap();
/// assert_eq!(0xFF, writer.into_inner()[0]);
///
/// let mut writer = io::Cursor::new(vec![]);
/// 0x1234_u16.write_integer::<BigEndian>(&mut writer).unwrap();
/// let result = writer.into_inner();
/// assert_eq!(0x12, result[0]);
/// assert_eq!(0x34, result[1]);
///
/// let mut writer = io::Cursor::new(vec![]);
/// (-2 as i16).write_integer::<BigEndian>(&mut writer).unwrap();
/// let result = writer.into_inner();
/// assert_eq!(0xFF, result[0]);
/// assert_eq!(0xFE, result[1]);
///
/// let mut writer = io::Cursor::new(vec![]);
/// 0x12345678_u32.write_integer::<BigEndian>(&mut writer).unwrap();
/// let result = writer.into_inner();
/// assert_eq!(0x12, result[0]);
/// assert_eq!(0x34, result[1]);
/// assert_eq!(0x56, result[2]);
/// assert_eq!(0x78, result[3]);
///
/// let mut writer = io::Cursor::new(vec![]);
/// (-3 as i32).write_integer::<BigEndian>(&mut writer).unwrap();
/// let result = writer.into_inner();
/// assert_eq!(0xFF, result[0]);
/// assert_eq!(0xFF, result[1]);
/// assert_eq!(0xFF, result[2]);
/// assert_eq!(0xFD, result[3]);
///
/// let mut writer = io::Cursor::new(vec![]);
/// 0x1234567812345678_u64.write_integer::<BigEndian>(&mut writer).unwrap();
/// let result = writer.into_inner();
/// assert_eq!(0x12, result[0]);
/// assert_eq!(0x34, result[1]);
/// assert_eq!(0x56, result[2]);
/// assert_eq!(0x78, result[3]);
/// assert_eq!(0x12, result[4]);
/// assert_eq!(0x34, result[5]);
/// assert_eq!(0x56, result[6]);
/// assert_eq!(0x78, result[7]);
///
/// let mut writer = io::Cursor::new(vec![]);
/// (-4 as i64).write_integer::<BigEndian>(&mut writer).unwrap();
/// let result = writer.into_inner();
/// assert_eq!(0xFF, result[0]);
/// assert_eq!(0xFF, result[1]);
/// assert_eq!(0xFF, result[2]);
/// assert_eq!(0xFF, result[3]);
/// assert_eq!(0xFF, result[4]);
/// assert_eq!(0xFF, result[5]);
/// assert_eq!(0xFF, result[6]);
/// assert_eq!(0xFC, result[7]);
///
/// ```
///
pub trait WriteInteger {
    /// Writes integer to the `writer`.
    fn write_integer<TEndian>(&self, writer: &mut io::Write) -> io::Result<()>
        where TEndian: Endian;
}

impl WriteInteger for u8 {
    fn write_integer<TEndian>(&self, writer: &mut io::Write) -> io::Result<()> where TEndian: Endian {
        let buf: &mut [u8] = &mut [0; 1];
        TEndian::u8_to_bytes(*self, buf);
        writer.write_all(buf)
    }
}

impl WriteInteger for i8 {
    fn write_integer<TEndian>(&self, writer: &mut io::Write) -> io::Result<()> where TEndian: Endian {
        u8::write_integer::<TEndian>(&(*self as u8), writer)
    }
}

impl WriteInteger for u16 {
    fn write_integer<TEndian>(&self, writer: &mut io::Write) -> io::Result<()> where TEndian: Endian {
        let buf: &mut [u8] = &mut [0; 2];
        TEndian::u16_to_bytes(*self, buf);
        writer.write_all(buf)
    }
}

impl WriteInteger for i16 {
    fn write_integer<TEndian>(&self, writer: &mut io::Write) -> io::Result<()> where TEndian: Endian {
        u16::write_integer::<TEndian>(&(*self as u16), writer)
    }
}

impl WriteInteger for u32 {
    fn write_integer<TEndian>(&self, writer: &mut io::Write) -> io::Result<()> where TEndian: Endian {
        let buf: &mut [u8] = &mut [0; 4];
        TEndian::u32_to_bytes(*self, buf);
        writer.write_all(buf)
    }
}

impl WriteInteger for i32 {
    fn write_integer<TEndian>(&self, writer: &mut io::Write) -> io::Result<()> where TEndian: Endian {
        u32::write_integer::<TEndian>(&(*self as u32), writer)
    }
}

impl WriteInteger for u64 {
    fn write_integer<TEndian>(&self, writer: &mut io::Write) -> io::Result<()> where TEndian: Endian {
        let buf: &mut [u8] = &mut [0; 8];
        TEndian::u64_to_bytes(*self, buf);
        writer.write_all(buf)
    }
}

impl WriteInteger for i64 {
    fn write_integer<TEndian>(&self, writer: &mut io::Write) -> io::Result<()> where TEndian: Endian {
        u64::write_integer::<TEndian>(&(*self as u64), writer)
    }
}

