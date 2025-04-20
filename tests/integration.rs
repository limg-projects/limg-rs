use limg::{Image, Result};
use std::io::Cursor;
use limg_core::decode_header;

#[test]
fn file_open_test() -> Result<()> {
    let dir = std::fs::read_dir("tests/limg")?;

    for item in dir.into_iter() {
        let path = item?.path();
        let data = std::fs::read(&path)?;

        let spec = decode_header(&data)?;
        
        let image = Image::open(path)?;

        let mut buf = Cursor::new(Vec::<u8>::new());
        image.to_write_with_endian(&mut buf, spec.pixel_endian)?;

        let buf = buf.into_inner();

        assert_eq!(data, buf);
    }

    Ok(())
}

#[test]
fn from_reader_test() -> Result<()> {
    let dir = std::fs::read_dir("tests/limg")?;

    for item in dir.into_iter() {
        let path = item?.path();
        let data = std::fs::read(&path)?;

        let spec = decode_header(&data)?;

        let file = std::fs::File::open(&path)?;
        let image = Image::from_read(file)?;

        let mut buf = Cursor::new(Vec::<u8>::new());
        image.to_write_with_endian(&mut buf, spec.pixel_endian)?;

        let buf = buf.into_inner();

        assert_eq!(data, buf);
    }

    Ok(())
}