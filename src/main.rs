use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Error as IoError, Read};
use thiserror::Error;

trait ReadSkip: Read {
    fn skip(&mut self, len: u64) -> Result<u64, IoError> {
        std::io::copy(&mut self.take(len), &mut std::io::sink())
    }
}

#[derive(Error, Debug)]
enum BmpError {
    #[error("type of this file is unsupported")]
    UnsupportedFileType,
    #[error("failed to read because: {0:?}")]
    ReadError(#[from] IoError),
}

#[derive(Debug)]
struct Bmp {
    image_size: u32,
    pixel_offset: u32,
}

impl Bmp {
    fn from_read<R: Read>(reader: &mut R) -> Result<Bmp, BmpError> {
        let mut header_buf = [0u8; 14];
        reader.read_exact(&mut header_buf)?;

        if header_buf[0] != 0x42 || header_buf[1] != 0x4D {
            return Err(BmpError::UnsupportedFileType);
        }

        let size = (&header_buf[2..=5]).read_u32::<LittleEndian>()?;
        let pixel_offset = (&header_buf[10..=13]).read_u32::<LittleEndian>()?;

        Ok(Bmp {
            image_size: size,
            pixel_offset,
        })
    }
}

fn main() {
    let mut pic = File::open("../test_picture/Lenna.bmp").unwrap();
    println!("{:?}", Bmp::from_read(&mut pic).unwrap());
}
