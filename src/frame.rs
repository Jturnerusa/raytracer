use std::ops::{Index, IndexMut};

pub trait Rgba32 {
    fn to_rgba32(&self) -> (u8, u8, u8, u8);
    fn from_rgba32(rgba: (u8, u8, u8, u8)) -> Self;
}

impl Rgba32 for sdl2::pixels::Color {
    fn to_rgba32(&self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    fn from_rgba32((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self::RGBA(r, g, b, a)
    }
}

impl Rgba32 for nalgebra::Vector3<f64> {
    fn to_rgba32(&self) -> (u8, u8, u8, u8) {
        (
            (self.x * 255.0) as u8,
            (self.y * 255.0) as u8,
            (self.z * 255.0) as u8,
            0,
        )
    }

    fn from_rgba32((r, g, b, _): (u8, u8, u8, u8)) -> Self {
        Self::new(r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
    }
}

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

    pub fn set_pixel<T: Rgba32>(&mut self, x: usize, y: usize, color: T) {
        let start = (x + y * self.width) * 4;
        let stop = start + 4;
        let (r, g, b, a) = color.to_rgba32();
        let bytes = [r, g, b, a];
        self.pixel_data[start..stop].copy_from_slice(bytes.as_slice());
    }

    pub fn get_pixel<T: Rgba32>(&self, x: usize, y: usize) -> T {
        let start = (x + y * self.width) * 4;
        let stop = start + 4;
        let [r, g, b, a]: [u8; 4] = self.pixel_data[start..stop].try_into().unwrap();
        T::from_rgba32((r, g, b, a))
    }
}
