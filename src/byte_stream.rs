#![allow(unstable)]

use std::old_io::{BufferedReader, IoError, IoErrorKind};

pub struct ByteStream<R: Reader> {
    reader: BufferedReader<R>,
}

impl<R: Reader> ByteStream<R> {
    #[inline]
    pub fn new(reader: R) -> ByteStream<R> {
        ByteStream {
            reader: BufferedReader::new(reader)
        }
    }
}

/// Return true if the io error is EOF.
fn is_eof(err: &IoError) -> bool {
    err.kind == IoErrorKind::EndOfFile
}

impl<R: Reader> Iterator for ByteStream<R> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<u8> {
        match self.reader.read_byte() {
            Ok(byte)                => Some(byte),
            Err(ref e) if is_eof(e) => None,
            Err(e)                  => panic!("IoError: {}!", e),
        }
    }
}
