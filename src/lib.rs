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

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
