### Simple image color extractor written in Rust with no external dependencies

Status:

[![Build Status](https://travis-ci.org/marekm4/dominant_color.svg?branch=master)](https://travis-ci.org/marekm4/dominant_color)

Demo:

http://dominant-color-demo.herokuapp.com/

Usage:
```rust
extern crate dominant_color;
extern crate image;

use std::path;

fn main() {
    let image = image::open(&path::Path::new("docs/Fotolia_45549559_320_480.jpg")).unwrap();
    let has_alpha = match image.color() {
        image::ColorType::Rgba8 => true,
        _ => false,
    };
    let colors = dominant_color::get_colors(&image.to_bytes(), has_alpha);
    println!("{:?}", colors);
}
```

Example image:

![Image](https://raw.githubusercontent.com/marekm4/dominant_color/master/docs/Fotolia_45549559_320_480.jpg)

Extracted colors:

![Colors](https://raw.githubusercontent.com/marekm4/dominant_color/master/docs/colors.png)
