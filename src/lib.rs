extern crate byteorder;
use byteorder::{BigEndian,ByteOrder};

struct PngHeader;

impl AsRef<[u8]> for PngHeader {
    fn as_ref(&self) -> &[u8] {
        &[ 0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A ]
    }
}

struct PngChunks<'a> {
    cur: usize,
    data: &'a [u8]
}

impl<'a> PngChunks<'a> {
    fn new(data: &[u8]) -> PngChunks {
        PngChunks {
            cur: 0,
            data: data
        }
    }
}

struct Chunk<'a> {
    siz: usize,
    typ: u32,
    dat: &'a [u8],
    crc: u32
}

impl<'a> AsRef<[u8]> for Chunk<'a> {
    fn as_ref(&self) -> &[u8] {
        &[ 0x01 ]
    }
}

impl<'a> Iterator for PngChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Chunk<'a>> {
        let siz = BigEndian::read_u32(&self.data[self.cur..]) as usize;
        self.cur += 4;
        let typ = BigEndian::read_u32(&self.data[self.cur..]);
        self.cur += 4;
        let dat = &self.data[self.cur..self.cur + siz];
        self.cur += siz;
        let crc = BigEndian::read_u32(&self.data[self.cur..]);
        self.cur += 4;

        Some(Chunk { siz, typ, dat, crc })
    }
}

struct Png<'a> {
    data: &'a [u8]
}

impl<'a> Png<'a> {
    fn new(data: &[u8]) -> Png {
        Png { data }
    }

    fn parts(&self) -> (PngHeader, PngChunks<'a>) {
        (PngHeader, PngChunks::new(&self.data[8..]))
    }
}

pub fn crop<T: AsRef<[u8]>>(input: T, output: &mut Vec<u8>) {
    let bytes = input.as_ref();
    let png = Png::new(bytes);
    let parts = png.parts();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
