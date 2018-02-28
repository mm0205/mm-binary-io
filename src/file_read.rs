//! Provides basic functions to read file.

use std::io;

use binary_read::BinaryRead;

pub trait FileRead: BinaryRead + io::Seek {

}