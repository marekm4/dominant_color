extern crate dominant_color;

#[test]
fn empty_image() {
    let pixels: [u8; 0] = [];
    let colors = dominant_color::get_colors(&pixels);
    assert_eq!(colors.len(), 0);
}
