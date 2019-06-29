extern crate dominant_color;

use std::path;

#[test]
fn empty_image() {
    let pixels: [u8; 0] = [];
    let colors = dominant_color::get_colors(&pixels);
    assert_eq!(colors.len(), 0);
}

#[test]
fn single_pixel() {
    let pixels: [u8; 3] = [135, 202, 82];
    let colors = dominant_color::get_colors(&pixels);
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
    let colors = dominant_color::get_colors(&pixels);
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
    let colors = dominant_color::get_colors(&pixels);
    assert_eq!(colors[0], dominant_color::Color { r: 255, g: 0, b: 0 });
    assert_eq!(colors[1], dominant_color::Color { r: 0, g: 255, b: 0 });
}

#[test]
fn mixed_colors() {
    let pixels: [u8; 21] = [
        255, 0, 0, 255, 0, 0, 245, 0, 0, 245, 0, 0, 0, 255, 0, 0, 255, 0, 0, 240, 0,
    ];
    let colors = dominant_color::get_colors(&pixels);
    assert_eq!(colors[0], dominant_color::Color { r: 250, g: 0, b: 0 });
    assert_eq!(colors[1], dominant_color::Color { r: 0, g: 250, b: 0 });
}