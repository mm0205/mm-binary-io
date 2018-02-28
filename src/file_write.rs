//! Provides basic functions to write file.

use std::io;

use binary_write::BinaryWrite;

pub trait FileWrite: BinaryWrite + io::Seek {

}
