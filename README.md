# limg
Limg 画像を読み書きするためのライブラリです。

## Features
このクレートは`#![no_std]`と互換性があります。変更するには`default-features = false`にしてください。
その場合、[`io`]の機能が制限されます。

`std`: std機能を提供します。（デフォルトでオンです。）

[`alloc`]: https://doc.rust-lang.org/alloc/
[`io`]: https://doc.rust-lang.org/std/io/index.html

## Usage 
`Cargo.toml`に以下を入れてください。

```toml
[dependencies]
limg = { git = "https://github.com/limg-projects/limg-rs", tag = "v0.1.0" }
```

## Examples

```rust,no_run
use limg::{Image, Pixel, Result, px};

fn main() -> Result<()> {
    let mut image = Image::new(256, 256);

    for (x, y) in image.coordinates() {
        image[(x, y)] = px!(x as u8, y as u8, 100);
    }
    
    image.save("image.limg")?;

    Ok(())
}
```