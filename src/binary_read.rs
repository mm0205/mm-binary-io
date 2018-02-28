use std::io;

use endian::Endian;
use read_integer::ReadInteger;

/// Provides the features to read binary data.
///
/// # Examples
///
/// ```
/// use std::io;
/// use mm_binary_io::endian::BigEndian;
/// use mm_binary_io::binary_read::BinaryRead;
///
/// let data = vec![0xFF_u8, 0xFF, 0xFF, 0xFE];
/// let mut reader = io::Cursor::new(data);
///
/// assert_eq!(-2_i32, reader.read_integer::<BigEndian,_>().unwrap());
///
///
/// let data = vec![0x12_u8, 0x32, 0x56, 0x78];
/// let mut reader = io::Cursor::new(data);
///
/// assert_eq!(vec![0x12, 0x32, 0x56], reader.read_byte_array(3).unwrap());
///
/// let data = vec![0x12_u8, 0x32, 0x56, 0x78, 0x90, 0x12, 0xFF];
/// let mut reader = io::Cursor::new(data);
///
/// assert_eq!(vec![0x1232_u16, 0x5678, 0x9012], reader.read_integer_array::<BigEndian, _>(3).unwrap());
///
/// ```
///
pub trait BinaryRead: io::Read {
    /// Reads an integer.
    ///
    /// # Errors
    ///
    /// If the function succeeds then Ok(TInt), otherwise Err(io::Error).
    ///
    fn read_integer<TEndian, TInt>(&mut self) -> io::Result<TInt>
        where
            TEndian: Endian,
            TInt: ReadInteger<OutputType=TInt>;

    /// Reads byte array.
    ///
    /// # Arguments
    ///
    /// * byte_count - the byte count of the array.
    ///
    /// # Errors
    ///
    /// If the function succeeds then Ok(TInt), otherwise Err(io::Error).
    ///
    fn read_byte_array(&mut self, byte_count: usize) -> io::Result<Vec<u8>> {
        let mut buf = vec![0_u8; byte_count];
        self.read_exact(&mut buf)
            .and_then(|_| {
                Ok(buf)
            })
    }

    /// Reads an integer array.
    ///
    /// # Arguments
    ///
    /// * element_count - the number of elements in the array.
    ///
    /// # Errors
    ///
    /// If the function succeeds then Ok(TInt), otherwise Err(io::Error).
    ///
    fn read_integer_array<TEndian, TInt>(&mut self, element_count: usize) -> io::Result<Vec<TInt>>
        where
            TEndian: Endian,
            TInt: ReadInteger<OutputType=TInt>;
}

impl<T> BinaryRead for T
    where T: io::Read {
    fn read_integer<TEndian, TInt>(&mut self) -> io::Result<TInt> where
        TEndian: Endian,
        TInt: ReadInteger<OutputType=TInt> {
        TInt::read_integer::<TEndian>(self)
    }

    fn read_integer_array<TEndian, TInt>(&mut self, element_count: usize) -> io::Result<Vec<TInt>> where
        TEndian: Endian,
        TInt: ReadInteger<OutputType=TInt> {
        let mut result = Vec::with_capacity(element_count);
        for _i in 0..element_count {
            result.push(self.read_integer::<TEndian, TInt>()?);
        }
        Ok(result)
    }
}
