use std::ops::{Index, IndexMut};

pub struct FrameBuffer {
    width: usize,
    height: usize,
    pixel_data: Box<[u8]>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixel_data: vec![0; width * height * 4].into_boxed_slice(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn pixel_data(&self) -> &[u8] {
        &self.pixel_data
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: sdl2::pixels::Color) {
        let start = (x + y * self.width) * 4;
        let stop = start + 4;
        let bytes = color
            .to_u32(
                &sdl2::pixels::PixelFormat::try_from(sdl2::pixels::PixelFormatEnum::RGBA32)
                    .unwrap(),
            )
            .to_le_bytes();
        self.pixel_data[start..stop].copy_from_slice(bytes.as_slice());
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> sdl2::pixels::Color {
        let start = (x + y * self.width) * 4;
        let stop = start + 4;
        let bytes: [u8; 4] = self.pixel_data[start..stop].try_into().unwrap();
        let i = u32::from_le_bytes(bytes);
        sdl2::pixels::Color::from_u32(
            &sdl2::pixels::PixelFormat::try_from(sdl2::pixels::PixelFormatEnum::RGBA32).unwrap(),
            i,
        )
    }
}
