# dominant_color
Simple image color extractor written in Rust with no external dependencies

## Install
```
cargo add dominant_color
```

## Usage
```rust
use std::path;

fn main() {
    let image = image::open(path::Path::new("docs/Fotolia_45549559_320_480.jpg")).unwrap();

    // if you are sure that you are using RGB image
    let colors = dominant_color::get_colors(image.as_bytes(), false);
    println!("{:?}", colors);

    // if you are not sure
    let colors = dominant_color::get_colors(image.to_rgb8().into_raw().as_slice(), false);
    println!("{:?}", colors);

    // if image has alpha channel
    let colors = dominant_color::get_colors(image.to_rgba8().into_raw().as_slice(), true);
    println!("{:?}", colors);
}
```

## Example
![Image](https://raw.githubusercontent.com/marekm4/dominant_color/master/docs/Fotolia_45549559_320_480.jpg)
![Colors](https://raw.githubusercontent.com/marekm4/dominant_color/master/docs/colors.png)

## Demo
https://dominant-color-demo.marekm4.com/
