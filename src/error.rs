pub type Result<T> = ::core::result::Result<T, Error>;

/// エンコードおよびデコード時に発生する可能性があるエラー
#[derive(Debug)]
pub enum Error {
    /// 画像の幅および高さが0です。
    /// 
    /// エンコード時にサイズが0になる設定はできません。
    ZeroImageDimensions,

    /// 入力バッファの長さが足りません。
    InputBufferTooSmall,

    /// 出力バッファの長さが足りません。
    OutputBufferTooSmall,

    /// 画像形式がサポートされていません。
    /// 
    /// デコード時に発生する可能性があります。
    UnsupportedFormat,

    /// IOエラー
    #[cfg(feature = "std")]
    IoError(std::io::Error)
}

impl ::core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::ZeroImageDimensions => limg_core::Error::ZeroImageDimensions.fmt(f),
            Error::InputBufferTooSmall => limg_core::Error::InputBufferTooSmall.fmt(f),
            Error::OutputBufferTooSmall => limg_core::Error::OutputBufferTooSmall.fmt(f),
            Error::UnsupportedFormat => limg_core::Error::UnsupportedFormat.fmt(f),
            #[cfg(feature = "std")]
            Error::IoError(err) => err.fmt(f),
        }
    }
}

impl From<limg_core::Error> for Error {
    fn from(err: limg_core::Error) -> Self {
        match err {
            limg_core::Error::ZeroImageDimensions => Error::ZeroImageDimensions,
            limg_core::Error::InputBufferTooSmall => Error::InputBufferTooSmall,
            limg_core::Error::OutputBufferTooSmall => Error::OutputBufferTooSmall,
            limg_core::Error::UnsupportedFormat => Error::UnsupportedFormat,
        }
    }
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl ::core::error::Error for Error {}