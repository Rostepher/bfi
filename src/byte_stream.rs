pub struct ByteStream<'r, R: Reader + 'r> {
    reader: &'r mut R,
}

impl<'r, R: Reader> ByteStream<'r, R> {
    #[inline]
    pub fn new(reader: &'r mut R) -> ByteStream<'r, R> {
        ByteStream { reader: reader }
    }

    pub fn next_byte(&mut self) -> Option<u8> {
        match self.reader.read_byte() {
            Ok(byte) => Some(byte),
            Err(_)   => None, // eof
        }
    }
}
