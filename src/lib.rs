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

impl<'a> Iterator for PngChunks<'a> {
    type Item = Chunk<'a>;

    fn next(&mut self) -> Option<Chunk<'a>> {
        let size = BigEndian::read_u32(&self.data[self.cur..]) as usize;
        let dat = &self.data[self.cur..self.cur + size];
        self.cur += size + 12;
        Some(Chunk { dat })
    }
}

struct Chunk<'a> {
    dat: &'a [u8]
}

impl<'a> Chunk<'a> {
    fn size(&self) -> usize {
        BigEndian::read_u32(self.dat) as usize
    }

    fn data(&self) -> &[u8] {
        let size = self.size();
        &self.dat[8..size]
    }

    fn crc(&self) -> u32 {
        let size = self.size();
        BigEndian::read_u32(&self.dat[8 + size..])
    }

    fn typ(&self) -> u32 {
        BigEndian::read_u32(&self.dat[4..])
    }
}

impl<'a> AsRef<[u8]> for Chunk<'a> {
    fn as_ref(&self) -> &[u8] {
        self.dat
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

pub struct Rect {
    x: u32,
    y: u32,
    w: u32,
    h: u32
}

pub fn crop<T: AsRef<[u8]>>(input: T, rect: Rect, output: &mut Vec<u8>) {
    let png = Png::new(input.as_ref());
    let parts = png.parts();

    // Write header to output
    // Iterate over chunks
        // Drop the part if needed
        // Crop the part
        // Write part to output
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
