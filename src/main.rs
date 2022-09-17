use byteorder::{LittleEndian, ReadBytesExt};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, RenderTarget};
use std::fs::File;
use std::io::{Error as IoError, Read, Seek, SeekFrom};
use std::time::Duration;
use thiserror::Error;

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
    pixels: Vec<Vec<Color>>,
}

impl Bmp {
    fn from_read<R: Read + Seek>(reader: &mut R) -> Result<Bmp, BmpError> {
        let mut header_buf = [0u8; 14];
        reader.read_exact(&mut header_buf)?;

        if header_buf[0] != 0x42 || header_buf[1] != 0x4D {
            return Err(BmpError::UnsupportedFileType);
        }

        let size = (&header_buf[2..=5]).read_u32::<LittleEndian>()?;
        let pixel_offset = (&header_buf[10..=13]).read_u32::<LittleEndian>()?;

        reader.seek(SeekFrom::Start(pixel_offset as u64)).unwrap();

        let mut buf = vec![vec![Color::WHITE; 256]; 256];
        for y in (0..256).rev() {
            for x in 0..256 {
                buf[y][x].b = reader.read_u8().unwrap();
                buf[y][x].g = reader.read_u8().unwrap();
                buf[y][x].r = reader.read_u8().unwrap();
            }
        }

        Ok(Bmp {
            image_size: size,
            pixel_offset,
            pixels: buf,
        })
    }
}

fn draw_pixel<T: RenderTarget>(canvas: &mut Canvas<T>, x: i32, y: i32, color: Color) {
    canvas.set_draw_color(color);
    canvas.draw_point((x, y)).unwrap();
}

fn main() {
    let mut pic = File::open("../test_picture/Lenna.bmp").unwrap();
    let bmp = Bmp::from_read(&mut pic).unwrap();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("bmp", 256, 256)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .map_err(|e| e.to_string())
        .unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        for y in 0..256 {
            for x in 0..256 {
                draw_pixel(&mut canvas, x, y, bmp.pixels[y as usize][x as usize]);
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(5, 0));
    }
}
