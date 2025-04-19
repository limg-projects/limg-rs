#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use crate::pixel::Pixel;
use crate::error::Result;
use core::ops::{Index, IndexMut};
use core::slice::{from_raw_parts, from_raw_parts_mut};
use limg_core::{ImageSpec, ColorType, PixelEndian, HEADER_SIZE, PIXEL_BYTES};
use limg_core::{decode_header, decode_data, decoded_size};
use limg_core::{encode_header, encode_data, encoded_size};

#[inline(always)]
const fn image_index(x: u16, y: u16, width: u16) -> usize {
    y as usize * width as usize + x as usize
}

/// Limg画像
/// 
/// Limg形式画像の初期化やピクセルデータの変更、読み取りや保存を提供します。
/// 
/// ピクセルデータは左上から右下への行優先でアクセスされ、`(0, 0)`は左上隅であると定義されます。
pub struct Image {
    /// 画像の幅
    width: u16,

    /// 画像の高さ
    height: u16,

    /// 透明色
    /// 
    /// 指定しない場合`None`
    transparent_color: Option<Pixel>,

    /// ピクセルデータ
    pixels: Box<[Pixel]>,
}

impl Image {
    /// `width`と`height`を指定してLimg画像を作成します。
    /// 
    /// 透明色なしの黒で初期化されます。
    #[inline]
    pub fn new(width: u16, height: u16) -> Image {
        Image {
            width,
            height,
            transparent_color: None,
            pixels: vec![Pixel::BLACK; width as usize * height as usize].into_boxed_slice()
        }
    }

    /// `width`と`height`、`transparent_color`を指定してLimg画像を作成します。
    /// 
    /// 黒で初期化されます。
    #[inline]
    pub fn with_transparent_color(width: u16, height: u16, transparent_color: Pixel) -> Image {
        Image {
            width,
            height,
            transparent_color: Some(transparent_color),
            pixels: vec![Pixel::BLACK; width as usize * height as usize].into_boxed_slice()
        }
    }

    /// 画像の幅を返します。
    #[inline(always)]
    pub fn width(&self) -> u16 {
        self.width
    }

    /// 画像の高さを返します。
    #[inline(always)]
    pub fn height(&self) -> u16 {
        self.height
    }

    /// 画像の透明色を返します。
    /// 
    /// 指定がない場合`None`になります。
    #[inline(always)]
    pub fn transparent_color(&self) -> Option<Pixel> {
        self.transparent_color
    }

    /// 画像の透明色を設定します。
    /// 
    /// 指定しない場合`None`を設定してください。
    #[inline(always)]
    pub fn set_transparent_color(&mut self, transparent_color: Option<Pixel>) {
        self.transparent_color = transparent_color;
    }

    /// `(x, y)`の位置のピクセルを設定します。
    /// 
    /// # Panics
    /// 
    /// `(x, y)`が`(width, height)`の範囲内にない場合はパニックになります。
    #[inline(always)]
    pub fn set_pixel(&mut self, x: u16, y: u16, pixel: Pixel) {
        self.pixels[image_index(x, y, self.width)] = pixel;
    }

    /// `(x, y)`の位置のピクセルの参照を取得します。
    /// 
    /// `(x, y)`が`(width, height)`の範囲内にない場合は`None`を返します。
    #[inline(always)]
    pub fn get_pixel(&self, x: u16, y: u16) -> Option<&Pixel> {
        if x < self.width || y < self.height {
            Some(unsafe { self.pixels.get_unchecked(image_index(x, y, self.width)) })
        } else {
            None
        }
    }

    /// チェックなしで`(x, y)`の位置のピクセルの参照を取得します。
    /// 
    /// チェックありの場合[`get_pixel`]を使用してください。
    /// 
    /// [`get_pixel`]: Image::get_pixel
    /// 
    /// # Safety
    /// 
    /// 範囲外のインデックスを使用してこの関数を呼び出すと、 結果の参照が使用されない場合でも未定義の動作になります。
    #[inline(always)]
    pub unsafe fn get_pixel_unchecked(&self, x: u16, y: u16) -> &Pixel {
        unsafe { self.pixels.get_unchecked(image_index(x, y, self.width)) }
    }


    /// `(x, y)`の位置のピクセルの可変参照を取得します。
    /// 
    /// `(x, y)`が`(width, height)`の範囲内にない場合は`None`を返します。
    #[inline(always)]
    pub fn get_pixel_mut(&mut self, x:u16, y: u16) -> Option<&mut Pixel> {
        if x < self.width || y < self.height {
            Some(unsafe { self.pixels.get_unchecked_mut(image_index(x, y, self.width)) })
        } else {
            None
        }
    }

    /// チェックなしで`(x, y)`の位置のピクセルの可変参照を取得します。
    /// 
    /// チェックありの場合[`get_pixel_mut`]を使用してください。
    /// 
    /// [`get_pixel_mut`]: Image::get_pixel_mut
    /// 
    /// # Safety
    /// 
    /// 範囲外のインデックスを使用してこの関数を呼び出すと、 結果の参照が使用されない場合でも未定義の動作になります。
    #[inline(always)]
    pub unsafe fn get_pixel_unchecked_mut(&mut self, x: u16, y: u16) -> &mut Pixel {
        unsafe { self.pixels.get_unchecked_mut(image_index(x, y, self.width)) }
    }

    /// 画像のピクセルデータのスライスを取得します。
    #[inline(always)]
    pub fn pixels(&self) -> &[Pixel] {
        &self.pixels
    }

    /// 画像のピクセルデータの可変スライスを取得します。
    #[inline(always)]
    pub fn pixels_mut(&mut self) -> &mut [Pixel] {
        &mut self.pixels
    }

    /// 指定した色で画像を塗りつぶします。
    #[inline(always)]
    pub fn fill(&mut self, pixel: Pixel) {
        self.pixels.fill(pixel);
    }

    /// `buf`から画像を読み取り、`Image`を作成します。
    /// 
    /// # Errors
    /// 
    /// 画像データが不正な場合、`Error`を返します。
    pub fn from_buffer(buf: impl AsRef<[u8]>) -> Result<Image> {
        let buf = buf.as_ref();

        // ヘッダーのデコード
        let spec = decode_header(&buf)?;

        // ピクセルデータデコード
        let pixels_size = decoded_size(&spec, ColorType::Rgb565);
        let mut pixels = Box::<[Pixel]>::new_uninit_slice(pixels_size);
        let pixels_slice = unsafe { from_raw_parts_mut(pixels.as_mut_ptr().cast::<u8>(), pixels_size) };
        decode_data(&buf[HEADER_SIZE..], pixels_slice, &spec, ColorType::Rgb565)?;
        
        Ok(Image {
            width: spec.width,
            height: spec.height,
            transparent_color: spec.transparent_color.map(|color| Pixel(color)),
            pixels: unsafe { pixels.assume_init() }
        })
    }

    /// 画像をエンコードし`buf`に書き込みます。
    /// 
    /// ピクセルはリトルエンディアンで書き込まれます。
    /// 
    /// # Errors
    /// 
    /// 画像データが不正な場合、`Error`を返します。
    #[inline(always)]
    pub fn to_buffer(&self, buf: &mut impl AsMut<[u8]>) -> Result<()> {
        self.to_buffer_with_endian(buf, PixelEndian::Little)
    }

    /// 画像を指定された`endian`でピクセルエンコードし`buf`に書き込みます。
    /// 
    /// # Errors
    /// 
    /// 画像データが不正な場合、`Error`を返します。
    pub fn to_buffer_with_endian(&self, buf: &mut impl AsMut<[u8]>, endian: PixelEndian) -> Result<()> {
        let buf = buf.as_mut();

        let spec = ImageSpec {
            width: self.width,
            height: self.height,
            transparent_color: self.transparent_color.map(|p| p.0),
            pixel_endian: endian
        };

        let data_slice = unsafe { from_raw_parts(self.pixels.as_ptr().cast::<u8>(), spec.num_pixels() * PIXEL_BYTES) };   

        // 画像のエンコード
        encode_header(buf, &spec)?;
        let buf = unsafe { buf.get_unchecked_mut(HEADER_SIZE..) };
        encode_data(data_slice, buf, &spec, ColorType::Rgb565)?;

        Ok(())
    }
}

impl Index<(u16, u16)> for Image {
    type Output = Pixel;

    #[inline(always)]
    fn index(&self, index: (u16, u16)) -> &Self::Output {
        &self.pixels[image_index(index.0, index.1, self.width)]
    }
}

impl IndexMut<(u16, u16)> for Image {
    #[inline(always)]
    fn index_mut(&mut self, index: (u16, u16)) -> &mut Self::Output {
        &mut self.pixels[image_index(index.0, index.1, self.width)]
    }
}

#[cfg(feature = "std")]
impl Image {
    /// `path`から画像を読み取り、`Image`を作成します。
    /// 
    /// # Errors
    /// 
    /// 画像データが不正かIO操作に失敗した場合、`Error`を返します。
    #[inline(always)]
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Image> {
        Image::from_read(std::fs::File::open(path)?)
    }

    /// `reader`から画像を読み取り、`Image`を作成します。
    /// 
    /// # Errors
    /// 
    /// 画像データが不正かIO操作に失敗した場合、`Error`を返します。
    pub fn from_read(reader: impl std::io::Read) -> Result<Image> {
        let mut reader = reader;
        
        // ヘッダーのデコード
        let mut header_buf = [0u8; HEADER_SIZE];
        reader.read_exact(&mut header_buf)?;
        let spec = decode_header(&header_buf)?;

        // バイナリピクセルデータ読み込み
        let data_size = spec.num_pixels() * PIXEL_BYTES;
        let mut data = Box::<[u8]>::new_uninit_slice(spec.num_pixels() * PIXEL_BYTES);
        let data_slice = unsafe { from_raw_parts_mut(data.as_mut_ptr().cast::<u8>(), data_size) };
        reader.read_exact(data_slice)?;

        // ピクセルデータデコード
        let pixels_size = decoded_size(&spec, ColorType::Rgb565);
        let mut pixels = Box::<[Pixel]>::new_uninit_slice(pixels_size);
        let pixels_slice = unsafe { from_raw_parts_mut(pixels.as_mut_ptr().cast::<u8>(), pixels_size) };
        decode_data(data_slice, pixels_slice, &spec, ColorType::Rgb565)?;
        
        Ok(Image {
            width: spec.width,
            height: spec.height,
            transparent_color: spec.transparent_color.map(|c| Pixel(c)),
            pixels: unsafe { pixels.assume_init() }
        })
    }

    /// 画像をエンコードし`path`に保存します。既にファイルが存在する場合上書きします。
    /// 
    /// ピクセルはリトルエンディアンで書き込まれます。
    /// 
    /// # Errors
    /// 
    /// 画像データが不正かIO操作に失敗した場合、`Error`を返します。
    #[inline(always)]
    pub fn save(&self, path: impl AsRef<std::path::Path>) -> Result<()> {
        let mut file = std::fs::File::create(path)?;
        self.to_write_with_endian(&mut file, PixelEndian::Little)
    }

    /// 画像を指定された`endian`でピクセルエンコードし`path`に保存します。既にファイルが存在する場合上書きします。
    /// 
    /// # Errors
    /// 
    /// 画像データが不正かIO操作に失敗した場合、`Error`を返します。
    #[inline(always)]
    pub fn save_with_endian(&self, path: impl AsRef<std::path::Path>, endian: PixelEndian) -> Result<()> {
        let mut file = std::fs::File::create(path)?;
        self.to_write_with_endian(&mut file, endian)
    }

    /// 画像をエンコードし`writer`に書き込みます。
    /// 
    /// ピクセルはリトルエンディアンで書き込まれます。
    /// 
    /// # Errors
    /// 
    /// 画像データが不正かIO操作に失敗した場合、`Error`を返します。
    #[inline(always)]
    pub fn to_write(&self, writer: &mut impl std::io::Write) -> Result<()> {
        self.to_write_with_endian(writer, PixelEndian::Little)
    }

    /// 画像を指定された`endian`でピクセルエンコードし`writer`に書き込みます。
    /// 
    /// # Errors
    /// 
    /// 画像データが不正かIO操作に失敗した場合、`Error`を返します。
    pub fn to_write_with_endian(&self, writer: &mut impl std::io::Write, endian: PixelEndian) -> Result<()> {
        let spec = ImageSpec {
            width: self.width,
            height: self.height,
            transparent_color: self.transparent_color.map(|p| p.0),
            pixel_endian: endian
        };

        // バッファの用意
        let buf_size = encoded_size(&spec);
        let mut encoded_buf = Box::<[u8]>::new_uninit_slice(buf_size);
        let buf_slice = unsafe { from_raw_parts_mut(encoded_buf.as_mut_ptr().cast::<u8>(), buf_size) };
        let data_slice = unsafe { from_raw_parts(self.pixels.as_ptr().cast::<u8>(), spec.num_pixels() * PIXEL_BYTES) };

        // 画像のエンコード
        encode_header(buf_slice, &spec)?;
        let buf_slice = unsafe { buf_slice.get_unchecked_mut(HEADER_SIZE..) };
        encode_data(data_slice, buf_slice, &spec, ColorType::Rgb565)?;

        // 書き込み
        writer.write_all(&buf_slice)?;
        writer.flush()?;

        Ok(())
    }
}