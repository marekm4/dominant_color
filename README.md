### Simple image color extractor written in Rust with no external dependencies

Demo:

https://dominant-color-demo.marekm4.com/

Blog post:

https://medium.com/@marek.michalik/c-vs-rust-vs-go-performance-analysis-945ab749056c

Usage:
```rust
use std::path;

fn main() {
    let image = image::open(path::Path::new("docs/Fotolia_45549559_320_480.jpg")).unwrap();
    let colors = dominant_color::get_colors(image.to_rgb8().into_raw().as_slice(), false);
    println!("colors: {:?}", colors);
}
```

Example image:

![Image](https://raw.githubusercontent.com/marekm4/dominant_color/master/docs/Fotolia_45549559_320_480.jpg)

Extracted colors:

![Colors](https://raw.githubusercontent.com/marekm4/dominant_color/master/docs/colors.png)
