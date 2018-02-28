//! Provides Byte operation for Big/Little Endians.

/// Provides functions to convert a byte array to integer, and vice versa.
///
/// # Examples
///
/// Converts byte array to integer.
///
/// ```
///
/// // If the bytes stored in Big Endian, use BigEndian, otherwise use LittleEndian.
/// use mm_binary_io::endian::{Endian, BigEndian};
///
/// assert_eq!(0x12, BigEndian::u8_from_bytes(&vec![0x12]));
/// assert_eq!(0x1234, BigEndian::u16_from_bytes(&vec![0x12, 0x34]));
/// assert_eq!(0x12345678, BigEndian::u32_from_bytes(&vec![0x12, 0x34, 0x56, 0x78]));
/// assert_eq!(0x1234567812345678, BigEndian::u64_from_bytes(&vec![0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78]));
///
/// assert_eq!(-2, BigEndian::i8_from_bytes(&vec![0xFE]));
/// assert_eq!(-3, BigEndian::i16_from_bytes(&vec![0xFF, 0xFD]));
/// assert_eq!(-4, BigEndian::i32_from_bytes(&vec![0xFF, 0xFF, 0xFF, 0xFC]));
/// assert_eq!(-5, BigEndian::i64_from_bytes(&vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFB ]));
///
/// ```
///
/// Converts integer to byte array.
///
/// ```
///
/// // If the bytes stored in Big Endian, use BigEndian, otherwise use LittleEndian.
/// use mm_binary_io::endian::{Endian, BigEndian};
///
/// let mut v_u8 = vec![0; 1];
/// BigEndian::u8_to_bytes(0x12, &mut v_u8);
/// assert_eq!(0x12, v_u8[0]);
///
/// let mut v_u16 = vec![0; 2];
/// BigEndian::u16_to_bytes(0x1234, &mut v_u16);
/// assert_eq!(vec![0x12, 0x34], v_u16);
///
/// let mut v_u32 = vec![0; 4];
/// BigEndian::u32_to_bytes(0x12345678, &mut v_u32);
/// assert_eq!(vec![0x12, 0x34, 0x56, 0x78], v_u32);
///
/// let mut v_u64 = vec![0; 8];
/// BigEndian::u64_to_bytes(0x1234567812345678, &mut v_u64);
/// assert_eq!(vec![0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78], v_u64);
///
/// let mut v_i8 = vec![0; 1];
/// BigEndian::i8_to_bytes(-1, &mut v_i8);
/// assert_eq!(0xFF, v_i8[0]);
///
/// let mut v_i16 = vec![0; 2];
/// BigEndian::i16_to_bytes(-2, &mut v_i16);
/// assert_eq!(vec![0xFF, 0xFE], v_i16);
///
/// let mut v_i32 = vec![0; 4];
/// BigEndian::i32_to_bytes(-3, &mut v_i32);
/// assert_eq!(vec![0xFF, 0xFF, 0xFF, 0xFD], v_i32);
///
/// let mut v_i64 = vec![0; 8];
/// BigEndian::i64_to_bytes(-4, &mut v_i64);
/// assert_eq!(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFC], v_i64);
///
/// ```
///
/// # Panics
///
/// If the bytes length is less than the size required by the integer, the functions panic.
/// The required size is equal to `std::mem::size_of::<Integer>()`
///
/// ```should_panic
///
/// use mm_binary_io::endian::{Endian, BigEndian};
///
/// BigEndian::u32_from_bytes(&vec![0x12]); // cause panic!
///
/// ```
///
///
pub trait Endian {
    /// Converts bytes to u8.
    fn u8_from_bytes(bytes: &[u8]) -> u8;

    /// Converts bytes to i8.
    fn i8_from_bytes(bytes: &[u8]) -> i8;

    /// Converts bytes to u16.
    fn u16_from_bytes(bytes: &[u8]) -> u16;

    /// Converts bytes to i16.
    fn i16_from_bytes(bytes: &[u8]) -> i16;

    /// Converts bytes to u32.
    fn u32_from_bytes(bytes: &[u8]) -> u32;

    /// Converts bytes to i32.
    fn i32_from_bytes(bytes: &[u8]) -> i32;

    /// Converts bytes to u64.
    fn u64_from_bytes(bytes: &[u8]) -> u64;

    /// Converts bytes to i64.
    fn i64_from_bytes(bytes: &[u8]) -> i64;

    /// Converts u8 to bytes.
    fn u8_to_bytes(value: u8, destination: &mut [u8]);

    /// Converts i8 to bytes.
    fn i8_to_bytes(value: i8, destination: &mut [u8]);

    /// Converts u16 to bytes.
    fn u16_to_bytes(value: u16, destination: &mut [u8]);

    /// Converts i16 to bytes.
    fn i16_to_bytes(value: i16, destination: &mut [u8]);

    /// Converts u32 to bytes.
    fn u32_to_bytes(value: u32, destination: &mut [u8]);

    /// Converts i32 to bytes.
    fn i32_to_bytes(value: i32, destination: &mut [u8]);

    /// Converts u64 to bytes.
    fn u64_to_bytes(value: u64, destination: &mut [u8]);

    /// Converts i64 to bytes.
    fn i64_to_bytes(value: i64, destination: &mut [u8]);
}

/// Provides functions to convert a byte array to integer, and vice versa for Big Endian.
pub struct BigEndian {}

impl Endian for BigEndian {
    fn u8_from_bytes(bytes: &[u8]) -> u8 {
        bytes[0]
    }

    fn i8_from_bytes(bytes: &[u8]) -> i8 {
        BigEndian::u8_from_bytes(bytes) as i8
    }

    fn u16_from_bytes(bytes: &[u8]) -> u16 {
        ((bytes[0] as u16) << 8)
            | ((bytes[1] as u16) << 0)
    }

    fn i16_from_bytes(bytes: &[u8]) -> i16 {
        BigEndian::u16_from_bytes(bytes) as i16
    }

    fn u32_from_bytes(bytes: &[u8]) -> u32 {
        ((bytes[0] as u32) << 24)
            | ((bytes[1] as u32) << 16)
            | ((bytes[2] as u32) << 8)
            | ((bytes[3] as u32) << 0)
    }

    fn i32_from_bytes(bytes: &[u8]) -> i32 {
        BigEndian::u32_from_bytes(bytes) as i32
    }

    fn u64_from_bytes(bytes: &[u8]) -> u64 {
        ((bytes[0] as u64) << 56)
            | ((bytes[1] as u64) << 48)
            | ((bytes[2] as u64) << 40)
            | ((bytes[3] as u64) << 32)
            | ((bytes[4] as u64) << 24)
            | ((bytes[5] as u64) << 16)
            | ((bytes[6] as u64) << 8)
            | ((bytes[7] as u64) << 0)
    }

    fn i64_from_bytes(bytes: &[u8]) -> i64 {
        BigEndian::u64_from_bytes(bytes) as i64
    }

    fn u8_to_bytes(value: u8, destination: &mut [u8]) {
        destination[0] = value
    }
    fn i8_to_bytes(value: i8, destination: &mut [u8]) {
        destination[0] = value as u8;
    }

    fn u16_to_bytes(value: u16, destination: &mut [u8]) {
        destination[0] = ((value >> 8) & 0xFF_u16) as u8;
        destination[1] = ((value >> 0) & 0xFF_u16) as u8;
    }

    fn i16_to_bytes(value: i16, destination: &mut [u8]) {
        let value = value as u16;
        destination[0] = ((value >> 8) & 0xFF_u16) as u8;
        destination[1] = ((value >> 0) & 0xFF_u16) as u8;
    }

    fn u32_to_bytes(value: u32, destination: &mut [u8]) {
        destination[0] = ((value >> 24) & 0xFF_u32) as u8;
        destination[1] = ((value >> 16) & 0xFF_u32) as u8;
        destination[2] = ((value >> 8) & 0xFF_u32) as u8;
        destination[3] = ((value >> 0) & 0xFF_u32) as u8;
    }

    fn i32_to_bytes(value: i32, destination: &mut [u8]) {
        let value = value as u32;
        destination[0] = ((value >> 24) & 0xFF_u32) as u8;
        destination[1] = ((value >> 16) & 0xFF_u32) as u8;
        destination[2] = ((value >> 8) & 0xFF_u32) as u8;
        destination[3] = ((value >> 0) & 0xFF_u32) as u8;
    }

    fn u64_to_bytes(value: u64, destination: &mut [u8]) {
        destination[0] = ((value >> 56) & 0xFF_u64) as u8;
        destination[1] = ((value >> 48) & 0xFF_u64) as u8;
        destination[2] = ((value >> 40) & 0xFF_u64) as u8;
        destination[3] = ((value >> 32) & 0xFF_u64) as u8;
        destination[4] = ((value >> 24) & 0xFF_u64) as u8;
        destination[5] = ((value >> 16) & 0xFF_u64) as u8;
        destination[6] = ((value >> 8) & 0xFF_u64) as u8;
        destination[7] = ((value >> 0) & 0xFF_u64) as u8;
    }

    fn i64_to_bytes(value: i64, destination: &mut [u8]) {
        let value = value as u64;
        destination[0] = ((value >> 56) & 0xFF_u64) as u8;
        destination[1] = ((value >> 48) & 0xFF_u64) as u8;
        destination[2] = ((value >> 40) & 0xFF_u64) as u8;
        destination[3] = ((value >> 32) & 0xFF_u64) as u8;
        destination[4] = ((value >> 24) & 0xFF_u64) as u8;
        destination[5] = ((value >> 16) & 0xFF_u64) as u8;
        destination[6] = ((value >> 8) & 0xFF_u64) as u8;
        destination[7] = ((value >> 0) & 0xFF_u64) as u8;
    }
}

/// Provides functions to convert a byte array to integer, and vice versa for Little Endian.
pub struct LittleEndian {}

impl Endian for LittleEndian {
    fn u8_from_bytes(bytes: &[u8]) -> u8 {
        bytes[0]
    }

    fn i8_from_bytes(bytes: &[u8]) -> i8 {
        BigEndian::u8_from_bytes(bytes) as i8
    }

    fn u16_from_bytes(bytes: &[u8]) -> u16 {
        ((bytes[1] as u16) << 8)
            | ((bytes[0] as u16) << 0)
    }

    fn i16_from_bytes(bytes: &[u8]) -> i16 {
        BigEndian::u16_from_bytes(bytes) as i16
    }

    fn u32_from_bytes(bytes: &[u8]) -> u32 {
        ((bytes[3] as u32) << 24)
            | ((bytes[2] as u32) << 16)
            | ((bytes[1] as u32) << 8)
            | ((bytes[0] as u32) << 0)
    }

    fn i32_from_bytes(bytes: &[u8]) -> i32 {
        BigEndian::u32_from_bytes(bytes) as i32
    }

    fn u64_from_bytes(bytes: &[u8]) -> u64 {
        ((bytes[7] as u64) << 56)
            | ((bytes[6] as u64) << 48)
            | ((bytes[5] as u64) << 40)
            | ((bytes[4] as u64) << 32)
            | ((bytes[3] as u64) << 24)
            | ((bytes[2] as u64) << 16)
            | ((bytes[1] as u64) << 8)
            | ((bytes[0] as u64) << 0)
    }

    fn i64_from_bytes(bytes: &[u8]) -> i64 {
        BigEndian::u64_from_bytes(bytes) as i64
    }

    fn u8_to_bytes(value: u8, destination: &mut [u8]) {
        destination[0] = value
    }
    fn i8_to_bytes(value: i8, destination: &mut [u8]) {
        LittleEndian::u8_to_bytes(value as u8, destination);
    }

    fn u16_to_bytes(value: u16, destination: &mut [u8]) {
        destination[1] = ((value >> 8) & 0xFF_u16) as u8;
        destination[0] = ((value >> 0) & 0xFF_u16) as u8;
    }

    fn i16_to_bytes(value: i16, destination: &mut [u8]) {
        let value = value as u16;
        destination[1] = ((value >> 8) & 0xFF_u16) as u8;
        destination[0] = ((value >> 0) & 0xFF_u16) as u8;
    }

    fn u32_to_bytes(value: u32, destination: &mut [u8]) {
        destination[3] = ((value >> 24) & 0xFF_u32) as u8;
        destination[2] = ((value >> 16) & 0xFF_u32) as u8;
        destination[1] = ((value >> 8) & 0xFF_u32) as u8;
        destination[0] = ((value >> 0) & 0xFF_u32) as u8;
    }

    fn i32_to_bytes(value: i32, destination: &mut [u8]) {
        let value = value as u32;
        destination[3] = ((value >> 24) & 0xFF_u32) as u8;
        destination[2] = ((value >> 16) & 0xFF_u32) as u8;
        destination[1] = ((value >> 8) & 0xFF_u32) as u8;
        destination[0] = ((value >> 0) & 0xFF_u32) as u8;
    }

    fn u64_to_bytes(value: u64, destination: &mut [u8]) {
        destination[7] = ((value >> 56) & 0xFF_u64) as u8;
        destination[6] = ((value >> 48) & 0xFF_u64) as u8;
        destination[5] = ((value >> 40) & 0xFF_u64) as u8;
        destination[4] = ((value >> 32) & 0xFF_u64) as u8;
        destination[3] = ((value >> 24) & 0xFF_u64) as u8;
        destination[2] = ((value >> 16) & 0xFF_u64) as u8;
        destination[1] = ((value >> 8) & 0xFF_u64) as u8;
        destination[0] = ((value >> 0) & 0xFF_u64) as u8;
    }

    fn i64_to_bytes(value: i64, destination: &mut [u8]) {
        let value = value as u64;
        destination[7] = ((value >> 56) & 0xFF_u64) as u8;
        destination[6] = ((value >> 48) & 0xFF_u64) as u8;
        destination[5] = ((value >> 40) & 0xFF_u64) as u8;
        destination[4] = ((value >> 32) & 0xFF_u64) as u8;
        destination[3] = ((value >> 24) & 0xFF_u64) as u8;
        destination[2] = ((value >> 16) & 0xFF_u64) as u8;
        destination[1] = ((value >> 8) & 0xFF_u64) as u8;
        destination[0] = ((value >> 0) & 0xFF_u64) as u8;
    }
}
