#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::Pixel;
use ::core::num::NonZeroU16;
use core::ops::Index;

pub struct Image {
    width: NonZeroU16,
    height: NonZeroU16,
    transparent_color: Pixel,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(width: NonZeroU16, height: NonZeroU16, transparent_color: Pixel) -> Image {
        Self::new_filled(width, height, transparent_color, Pixel::BLACK)
    }

    pub fn new_filled(width: NonZeroU16, height: NonZeroU16, transparent_color: Pixel, color: Pixel) -> Image {
        Image { width, height, transparent_color, pixels: vec![color; width.get() as usize * height.get() as usize] }
    }

    pub fn new_filled_with_transparent_color(width: NonZeroU16, height: NonZeroU16, transparent_color: Pixel) -> Image {
        Self::new_filled(width, height, transparent_color, transparent_color)
    }

    pub fn width(&self) -> NonZeroU16 {
        self.width
    }

    pub fn height(&self) -> NonZeroU16 {
        self.height
    }

    pub fn transparent_color(&self) -> Pixel {
        self.transparent_color
    }

    pub fn set_transparent_color(&mut self, transparent_color: Pixel) {
        self.transparent_color = transparent_color;
    }

    pub fn change_transparent_color(&mut self, transparent_color: Pixel) {
        if self.transparent_color == transparent_color {
            return;
        }

        for pixel in &mut self.pixels {
            if *pixel == self.transparent_color {
                *pixel = transparent_color;
            }
        }

        self.transparent_color = transparent_color;
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, color: Pixel) {
        self.pixels[y as usize * self.width.get() as usize + x as usize] = color;
    }

    pub fn set_pixel_with_transparent_color(&mut self, x: u16, y: u16) {
        self.set_pixel(x, y, self.transparent_color);
    }

    pub fn get_pixel(&self, x: u16, y: u16) -> Option<&Pixel> {
        if x >= self.width.get() {
            return None;
        }
        if y >= self.height.get() {
            return None;
        }
        Some(unsafe { self.get_pixel_unchecked(x, y) })
    }

    pub unsafe fn get_pixel_unchecked(&self, x: u16, y: u16) -> &Pixel {
        unsafe { self.pixels.get_unchecked(y as usize * self.width.get() as usize + x as usize) }
    }

    pub fn get_pixel_mut(&mut self, x:u16, y: u16) -> Option<&mut Pixel> {
        self.pixels.get_mut(y as usize * self.width.get() as usize + x as usize)
    }

    pub fn get_pixels(&self) -> &[Pixel] {
        &self.pixels
    }

    pub fn get_pixels_mut(&mut self) -> &mut [Pixel] {
        &mut self.pixels
    }

    pub fn fill(&mut self, color: Pixel) {
        self.pixels.fill(color);
    }

    pub fn fill_transparent_color(&mut self) {
        self.fill(self.transparent_color);
    }
}

impl Index<(u16, u16)> for Image {
    type Output = Pixel;

    fn index(&self, index: (u16, u16)) -> &Self::Output {
        if let Some(pixel) = self.get_pixel(index.0, index.1) {
            pixel
        } else {
            panic!(
                "index out of bounds: the len is {} * {}, but the index is ({}, {})",
                self.width.get(),
                self.height.get(),
                index.0,
                index.1
            )
        }
    }
}