use limg::{Image, Result};

#[test]
fn file_open_test() -> Result<()> {
    let dir = std::fs::read_dir("tests/limg")?;

    for item in dir.into_iter() {
        let path = item?.path();
        let _image = Image::open(path)?;
    }

    Ok(())
}

#[test]
fn from_reader_test() -> Result<()> {
    let dir = std::fs::read_dir("tests/limg")?;

    for item in dir.into_iter() {
        let path = item?.path();
        let file = std::fs::File::open(path)?;
        let _image = Image::from_read(file)?;
    }

    Ok(())
}