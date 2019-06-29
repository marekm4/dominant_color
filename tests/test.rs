extern crate dominant_color;
extern crate image;

use std::path;

#[test]
fn empty_image() {
    let pixels: [u8; 0] = [];
    let colors = dominant_color::get_colors(&pixels, false);
    assert_eq!(colors.len(), 0);
}

#[test]
fn transparent_image() {
    let pixels: [u8; 4] = [135, 202, 82, 0];
    let colors = dominant_color::get_colors(&pixels, true);
    assert_eq!(colors.len(), 0);
}

#[test]
fn single_pixel() {
    let pixels: [u8; 3] = [135, 202, 82];
    let colors = dominant_color::get_colors(&pixels, false);
    assert_eq!(
        colors[0],
        dominant_color::Color {
            r: 135,
            g: 202,
            b: 82
        }
    );
}

#[test]
fn one_color() {
    let pixels: [u8; 6] = [135, 202, 82, 135, 202, 82];
    let colors = dominant_color::get_colors(&pixels, false);
    assert_eq!(
        colors[0],
        dominant_color::Color {
            r: 135,
            g: 202,
            b: 82
        }
    );
}

#[test]
fn two_colors() {
    let pixels: [u8; 9] = [255, 0, 0, 0, 255, 0, 255, 0, 0];
    let colors = dominant_color::get_colors(&pixels, false);
    assert_eq!(colors[0], dominant_color::Color { r: 255, g: 0, b: 0 });
    assert_eq!(colors[1], dominant_color::Color { r: 0, g: 255, b: 0 });
}

#[test]
fn two_colors_and_one_transparent() {
    let pixels: [u8; 16] = [255, 0, 0, 255, 0, 255, 0, 255, 255, 0, 0, 255, 0, 0, 255, 0];
    let colors = dominant_color::get_colors(&pixels, true);
    assert_eq!(colors[0], dominant_color::Color { r: 255, g: 0, b: 0 });
    assert_eq!(colors[1], dominant_color::Color { r: 0, g: 255, b: 0 });
}

#[test]
fn mixed_colors() {
    let pixels: [u8; 21] = [
        255, 0, 0, 255, 0, 0, 245, 0, 0, 245, 0, 0, 0, 255, 0, 0, 255, 0, 0, 240, 0,
    ];
    let colors = dominant_color::get_colors(&pixels, false);
    assert_eq!(colors[0], dominant_color::Color { r: 250, g: 0, b: 0 });
    assert_eq!(colors[1], dominant_color::Color { r: 0, g: 250, b: 0 });
}

#[test]
fn image() {
    let image = image::open(&path::Path::new("docs/Fotolia_45549559_320_480.jpg")).unwrap();
    let colors = dominant_color::get_colors(&image.raw_pixels(), false);
    assert_eq!(
        colors[0],
        dominant_color::Color {
            r: 232,
            g: 230,
            b: 228
        }
    );
    assert_eq!(
        colors[1],
        dominant_color::Color {
            r: 58,
            g: 58,
            b: 10
        }
    );
    assert_eq!(
        colors[2],
        dominant_color::Color {
            r: 204,
            g: 52,
            b: 25
        }
    );
    assert_eq!(
        colors[3],
        dominant_color::Color {
            r: 191,
            g: 178,
            b: 56
        }
    );
    assert_eq!(
        colors[4],
        dominant_color::Color {
            r: 104,
            g: 152,
            b: 12
        }
    );
}
