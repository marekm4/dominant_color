### Simple image color extractor written in Rust with no external dependencies

Status:

[![Build Status](https://travis-ci.org/marekm4/dominant_color.svg?branch=master)](https://travis-ci.org/marekm4/dominant_color)

Demo:

http://dominant-color-demo.herokuapp.com/

Blog post:

https://medium.com/@marek.michalik/c-vs-rust-vs-go-performance-analysis-945ab749056c

Usage:
```rust
use std::path;

fn main() {
    let image = image::open(&path::Path::new("./docs/Fotolia_45549559_320_480.jpg")).unwrap();
    let has_alpha = match image.color() {
        image::ColorType::Rgba8 => true,
        image::ColorType::Bgra8 => true,
        _ => false,
    };
    let colors = dominant_color::get_colors(&image.to_bytes(), has_alpha);
    println!("has_alpha: {}, colors: {:?}", has_alpha, colors);
}
```

Example image:

![Image](https://raw.githubusercontent.com/marekm4/dominant_color/master/docs/Fotolia_45549559_320_480.jpg)

Extracted colors:

![Colors](https://raw.githubusercontent.com/marekm4/dominant_color/master/docs/colors.png)
