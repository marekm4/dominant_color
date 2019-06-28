extern crate dominant_color;

use dominant_color::Color;

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
