extern crate byteorder;
use byteorder::{BigEndian,ByteOrder};
use std::fmt::Display;
use std::fmt;

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

    fn validate(&self) {
        assert_eq!(&self.data[..8], PngHeader.as_ref());
    }
}

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
        if self.cur == self.data.len() {
            return None
        }

        let mut size = BigEndian::read_u32(&self.data[self.cur..]) as usize;
        size += 12;
        let data = &self.data[self.cur..self.cur + size];
        self.cur += size;
        Some(Chunk::new(data))
    }
}

struct Chunk<'a> {
    dat: &'a [u8]
}

impl<'a> Chunk<'a> {
    fn new(data: &'a [u8]) -> Chunk {
        Chunk { dat: data }
    }

    fn size(&self) -> usize {
        BigEndian::read_u32(self.dat) as usize
    }

    fn data(&self) -> &[u8] {
        let size = self.size();
        &self.dat[8..size + 8]
    }

    fn crc(&self) -> u32 {
        let size = self.size();
        BigEndian::read_u32(&self.dat[8 + size..])
    }

    fn typ(&self) -> u32 {
        BigEndian::read_u32(&self.dat[4..])
    }
}

impl<'a> Display for Chunk<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_string = String::from_utf8_lossy(&self.dat[4..8]);
        write!(f, "Chunk {{ Size: {}, Type: {}, CRC: {} }}", self.size(), type_string, self.crc())
    }
}

impl<'a> AsRef<[u8]> for Chunk<'a> {
    fn as_ref(&self) -> &[u8] {
        self.dat
    }
}

pub struct Rect {
    x: u32,
    y: u32,
    w: u32,
    h: u32
}

impl Rect {
    pub fn new(x: u32, y: u32, w: u32, h: u32) -> Rect {
        Rect { x, y, w, h }
    }
}

pub fn crop<T: AsRef<[u8]>>(input: T, rect: &Rect, output: &mut Vec<u8>) {
    let png = Png::new(input.as_ref());
    png.validate();
    println!("PNG header validated");
    let (header, chunks) = png.parts();

    // Write header to output
    output.extend_from_slice(header.as_ref());
    println!("Output header {:?}", output);

    // Write chunks to output
    chunks
        .inspect(|chunk| {
            println!("{}", chunk);
        })
        .filter(|chunk| can_output(&chunk))
        .for_each(|chunk| crop_chunk(&chunk, &rect, output));
    output.shrink_to_fit();
}

fn can_output(chunk: &Chunk) -> bool {
    chunk.typ() & 0x0F == 0x0F
}

const IHDR: u32 = 0x01;
const IDAT: u32 = 0x02;

fn crop_chunk<'a>(chunk: &Chunk<'a>, rect: &Rect, output: &mut Vec<u8>) {
    match chunk.typ() {
        IHDR => ihdr(chunk, rect, output),
        IDAT => idat(chunk, rect, output),
        _    => output.extend_from_slice(chunk.as_ref())
    }
}

fn ihdr<'a>(chunk: &Chunk<'a>, rect: &Rect, output: &mut Vec<u8>) {
    output.extend_from_slice(chunk.as_ref());
}

fn idat<'a>(chunk: &Chunk<'a>, rect: &Rect, output: &mut Vec<u8>) {
    output.extend_from_slice(chunk.as_ref());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
