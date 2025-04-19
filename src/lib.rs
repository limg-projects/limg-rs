#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod pixel;
mod image;
mod error;

pub use limg_core::PixelEndian;
pub use pixel::Pixel;
pub use image::Image;
pub use error::{Error, Result};
