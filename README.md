### Simple image color extractor written in Rust with no external dependencies
It's a Rust port of https://github.com/marekm4/color-extractor but **only RGB** palettes are supported for now!

Status:

[![Build Status](https://travis-ci.org/marekm4/dominant_color.svg?branch=master)](https://travis-ci.org/marekm4/dominant_color)

Demo:

https://color-extractor-demo.herokuapp.com/

Usage:
```rust
extern crate dominant_color;
extern crate image;

fn main() {
    let image = image::open(&path::Path::new("docs/Fotolia_45549559_320_480.jpg")).unwrap();
    let colors = dominant_color::get_colors(&image.raw_pixels());
    println!("{:?}", colors);
}
```

Example image:

![Image](https://raw.githubusercontent.com/marekm4/dominant_color/master/docs/Fotolia_45549559_320_480.jpg)

Extracted colors:

![Colors](https://raw.githubusercontent.com/marekm4/dominant_color/master/docs/colors.png)
