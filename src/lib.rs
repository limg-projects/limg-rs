#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod pixel;
mod image;
mod error;

pub use limg_core::PixelEndian;
pub use pixel::Pixel;
pub use image::{Image, ImageIndex};
pub use error::{Error, Result};
