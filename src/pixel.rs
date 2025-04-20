use ::core::fmt::*;
use limg_core::{pixel_to_rgb, rgb_to_pixel};

/// ピクセル生成マクロです。
/// 
/// # Examples
/// 
/// ```
/// # use limg::{Pixel, px};
/// assert_eq!(px!(0x0000), Pixel::new(0x0000));
/// assert_eq!(px!(0, 128, 255), Pixel::from_rgb([0, 128, 255]));
/// ```
#[macro_export]
macro_rules! px {
    ($color:expr) => {
        $crate::Pixel::new($color)
    };
    ($r:expr, $g:expr, $b:expr) => {
        $crate::Pixel::from_rgb([$r, $g, $b])
    };
}

/// Limg画像で使用するピクセルです。
/// 
/// RGB565ピクセルと同等です。
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pixel(pub u16);

impl Pixel {
    const R_MASK: u16 = 0xF800;
    const G_MASK: u16 = 0x07E0;
    const B_MASK: u16 = 0x001F;

    /// 黒のピクセル
    pub const BLACK:   Pixel = px!(0x0000);
    /// 赤のピクセル
    pub const RED:     Pixel = px!(0xF800);
    /// 緑のピクセル
    pub const GREEN:   Pixel = px!(0x07E0);
    /// 青のピクセル
    pub const BLUE:    Pixel = px!(0x001F);
    /// マゼンタのピクセル
    pub const MAGENTA: Pixel = px!(0xF81F);
    /// シアンのピクセル
    pub const CYAN:    Pixel = px!(0x07FF);
    /// 黄色のピクセル
    pub const YELLOW:  Pixel = px!(0xFFE0);
    /// 灰色のピクセル
    pub const GRAY:    Pixel = px!(0x7BEF);
    /// 白のピクセル
    pub const WHITE:   Pixel = px!(0xFFFF);

    /// RGB565でピクセルを生成します。
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use limg::Pixel;
    /// let pixel = Pixel::new(0x1234);
    /// 
    /// assert_eq!(pixel.0, 0x1234);
    /// ```
    pub const fn new(color: u16) -> Pixel {
        Pixel(color)
    }

    /// ピクセルから赤情報を取得します。
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use limg::Pixel;
    /// let pixel = Pixel::RED;
    /// 
    /// assert_eq!(pixel.r(), 255);
    /// ```
    pub const fn r(&self) -> u8 {
        let r = ((self.0 & Self::R_MASK) >> 11) as u8;
        (r << 3) | (r >> 2)
    }

    /// ピクセルから緑情報を取得します。
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use limg::Pixel;
    /// let pixel = Pixel::GREEN;
    /// 
    /// assert_eq!(pixel.g(), 255);
    /// ```
    pub const fn g(&self) -> u8 {
        let g = ((self.0 & Self::G_MASK) >> 5) as u8;
        (g << 2) | (g >> 4)
    }

    /// ピクセルから青情報を取得します。
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use limg::Pixel;
    /// let pixel = Pixel::BLUE;
    /// 
    /// assert_eq!(pixel.b(), 255);
    /// ```
    pub const fn b(&self) -> u8 {
        let b = (self.0 & Self::B_MASK) as u8;
        (b << 3) | (b >> 2)
    }

    /// ピクセルに赤情報を設定します。
    /// 
    /// 色は5ビットに減色されます。
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use limg::Pixel;
    /// let mut pixel = Pixel::BLACK;
    /// pixel.set_r(255);
    /// 
    /// assert_eq!(pixel, Pixel::RED);
    /// ```
    pub fn set_r(&mut self, r: u8) {
        let r = ((r as u16) << (11 - 3)) & Self::R_MASK;
        self.0 = r | (self.0 & !Self::R_MASK);
    }

    /// ピクセルに緑情報を設定します。
    /// 
    /// 色は6ビットに減色されます。
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use limg::Pixel;
    /// let mut pixel = Pixel::BLACK;
    /// pixel.set_g(255);
    /// 
    /// assert_eq!(pixel, Pixel::GREEN);
    /// ```
    pub fn set_g(&mut self, g: u8) {
        let g = ((g as u16) << (5 - 2)) & Self::G_MASK;
        self.0 = g | (self.0 & !Self::G_MASK);
    }

    /// ピクセルに青情報を設定します。
    /// 
    /// 色は5ビットに減色されます。
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use limg::Pixel;
    /// let mut pixel = Pixel::BLACK;
    /// pixel.set_b(255);
    /// 
    /// assert_eq!(pixel, Pixel::BLUE);
    /// ```
    pub fn set_b(&mut self, b: u8) {
        let b = (b as u16) >> 3;
        self.0 = b | (self.0 & !Self::B_MASK);
    }

    /// `[r, g, b]`配列からピクセルを生成します。
    /// 
    /// 色情報はRGB565に減色されます。
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use limg::Pixel;
    /// let pixel = Pixel::from_rgb([255, 0, 255]);
    /// 
    /// assert_eq!(pixel, Pixel::MAGENTA);
    /// ```
    pub const fn from_rgb(rgb: [u8; 3]) -> Pixel {
        Pixel(rgb_to_pixel(rgb))
    }

    /// ピクセルを`[r, g, b]`に変換します。
    /// 
    /// 色情報はRGB88で返されます。
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use limg::Pixel;
    /// let pixel = Pixel::MAGENTA;
    /// 
    /// assert_eq!(pixel.into_rgb(), [255, 0, 255]);
    /// ```
    pub const fn into_rgb(self) -> [u8; 3] {
        pixel_to_rgb(self.0)
    }
}

impl From<u16> for Pixel {
    fn from(color: u16) -> Self {
        px!(color)
    }
}

impl Debug for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Pixel({:#06X})", self.0)
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "rgb({}, {}, {})", self.r(), self.g(), self.b())
    }
}

impl Binary for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Binary::fmt(&self.0, f)
    }
}

impl LowerHex for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        LowerHex::fmt(&self.0, f)
    }
}

impl UpperHex for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        UpperHex::fmt(&self.0, f)
    }
}
