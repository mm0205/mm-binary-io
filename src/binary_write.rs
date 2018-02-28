//! Provides the features to write binary data.

use std::io;

use endian::Endian;
use write_integer::WriteInteger;

/// Provides the features to write binary data.
///
/// # Examples
///
/// ```
/// use std::io;
/// use mm_binary_io::endian::BigEndian;
/// use mm_binary_io::binary_write::BinaryWrite;
///
/// let mut writer = io::Cursor::new(vec![]);
/// let v_u8 = 0x12_u8;
/// writer.write_integer::<BigEndian, _>(v_u8).unwrap();
/// let result = writer.into_inner();
/// assert_eq!(v_u8, result[0]);
///
/// let mut writer = io::Cursor::new(vec![]);
/// let v_i8 = -2 as i8;
/// writer.write_integer::<BigEndian, _>(v_i8).unwrap();
/// let result = writer.into_inner();
/// assert_eq!(0xFE, result[0]);
///
/// let mut writer = io::Cursor::new(vec![]);
/// let v_u16 = 0x1234 as u16;
/// writer.write_integer::<BigEndian, _>(v_u16).unwrap();
/// let result = writer.into_inner();
/// assert_eq!(0x12, result[0]);
/// assert_eq!(0x34, result[1]);
///
/// let mut writer = io::Cursor::new(vec![]);
/// let v_i16 = -3 as i16;
/// writer.write_integer::<BigEndian, _>(v_i16).unwrap();
/// let result = writer.into_inner();
/// assert_eq!(0xFF, result[0]);
/// assert_eq!(0xFD, result[1]);
///
/// let mut writer = io::Cursor::new(vec![]);
/// let v_u32 = 0x12345678 as u32;
/// writer.write_integer::<BigEndian, _>(v_u32).unwrap();
/// let result = writer.into_inner();
/// assert_eq!(0x12, result[0]);
/// assert_eq!(0x34, result[1]);
/// assert_eq!(0x56, result[2]);
/// assert_eq!(0x78, result[3]);
///
/// let mut writer = io::Cursor::new(vec![]);
/// let v_i32 = -4 as i32;
/// writer.write_integer::<BigEndian, _>(v_i32).unwrap();
/// let result = writer.into_inner();
/// assert_eq!(0xFF, result[0]);
/// assert_eq!(0xFF, result[1]);
/// assert_eq!(0xFF, result[2]);
/// assert_eq!(0xFC, result[3]);
///
/// let mut writer = io::Cursor::new(vec![]);
/// let v_u64 = 0x1234567812345678 as u64;
/// writer.write_integer::<BigEndian, _>(v_u64).unwrap();
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
/// let v_i64 = -5 as i64;
/// writer.write_integer::<BigEndian, _>(v_i64).unwrap();
/// let result = writer.into_inner();
/// assert_eq!(0xFF, result[0]);
/// assert_eq!(0xFF, result[1]);
/// assert_eq!(0xFF, result[2]);
/// assert_eq!(0xFF, result[3]);
/// assert_eq!(0xFF, result[4]);
/// assert_eq!(0xFF, result[5]);
/// assert_eq!(0xFF, result[6]);
/// assert_eq!(0xFB, result[7]);
///
/// ```
pub trait BinaryWrite: io::Write {
    /// Writes the `value`.
    fn write_integer<TEndian, TInt>(&mut self, value: TInt) -> io::Result<()>
        where TEndian: Endian,
              TInt: WriteInteger;

    /// Writes the integer array.
    fn write_integer_array<TEndian, TInt>(&mut self, values: &[TInt]) -> io::Result<()>
        where TEndian: Endian,
              TInt: WriteInteger;
}

impl<T> BinaryWrite for T
    where T: io::Write {
    fn write_integer<TEndian, TInt>(&mut self, value: TInt) -> io::Result<()>
        where TEndian: Endian,
              TInt: WriteInteger {
        value.write_integer::<TEndian>(self)
    }

    fn write_integer_array<TEndian, TInt>(&mut self, values: &[TInt]) -> io::Result<()>
        where TEndian: Endian,
              TInt: WriteInteger {

        for x in values {
            x.write_integer::<TEndian>(self)?;
        }
        Ok(())
    }
}
