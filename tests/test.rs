use std::path;

#[test]
fn empty_image() {
    let pixels: Vec<u8> = vec![];
    let colors = dominant_color::get_colors(&pixels, false);
    assert_eq!(colors.len(), 0 * 3);
}

#[test]
fn transparent_image() {
    let pixels: Vec<u8> = vec![135, 202, 82, 0];
    let colors = dominant_color::get_colors(&pixels, true);
    assert_eq!(colors.len(), 0 * 4);
}

#[test]
fn semi_transparent_single_pixel() {
    let pixels: Vec<u8> = vec![135, 202, 82, 127];
    let colors = dominant_color::get_colors(&pixels, true);
    assert_eq!(colors.len(), 1 * 4);
    assert_eq!(colors, vec!(135, 202, 82, 255));
}

#[test]
fn semi_transparent_image() {
    let pixels: Vec<u8> = vec![255, 0, 0, 127, 0, 255, 0, 255];
    let colors = dominant_color::get_colors(&pixels, true);
    assert_eq!(colors.len(), 2 * 4);
    assert_eq!(colors, vec!(0, 255, 0, 255, 255, 0, 0, 255));
}

#[test]
fn semi_transparent_region_is_bigger() {
    let pixels: Vec<u8> = vec![
        255, 0, 0, 127, 255, 0, 0, 127, 255, 0, 0, 127, 0, 255, 0, 255,
    ];
    let colors = dominant_color::get_colors(&pixels, true);
    assert_eq!(colors.len(), 2 * 4);
    assert_eq!(colors, vec!(255, 0, 0, 255, 0, 255, 0, 255));
}

#[test]
fn single_pixel() {
    let pixels: Vec<u8> = vec![135, 202, 82];
    let colors = dominant_color::get_colors(&pixels, false);
    assert_eq!(colors.len(), 3);
    assert_eq!(colors, vec!(135, 202, 82));
}

#[test]
fn one_color() {
    let pixels: Vec<u8> = vec![135, 202, 82, 135, 202, 82];
    let colors = dominant_color::get_colors(&pixels, false);
    assert_eq!(colors.len(), 3);
    assert_eq!(colors, vec!(135, 202, 82));
}

#[test]
fn two_colors() {
    let pixels: Vec<u8> = vec![255, 0, 0, 0, 255, 0, 255, 0, 0];
    let colors = dominant_color::get_colors(&pixels, false);
    assert_eq!(colors.len(), 2 * 3);
    assert_eq!(colors, vec!(255, 0, 0, 0, 255, 0));
}

#[test]
fn two_colors_and_one_transparent() {
    let pixels: Vec<u8> = vec![255, 0, 0, 255, 0, 255, 0, 255, 255, 0, 0, 255, 0, 0, 255, 0];
    let colors = dominant_color::get_colors(&pixels, true);
    assert_eq!(colors.len(), 2 * 4);
    assert_eq!(colors, vec!(255, 0, 0, 255, 0, 255, 0, 255));
}

#[test]
fn mixed_colors() {
    let pixels: Vec<u8> = vec![
        255, 0, 0, 255, 0, 0, 245, 0, 0, 245, 0, 0, 0, 255, 0, 0, 255, 0, 0, 240, 0,
    ];
    let colors = dominant_color::get_colors(&pixels, false);
    assert_eq!(colors.len(), 2 * 3);
    assert_eq!(colors, vec!(250, 0, 0, 0, 250, 0));
}

#[test]
fn image() {
    let image = image::open(path::Path::new("docs/Fotolia_45549559_320_480.jpg")).unwrap();
    let colors = dominant_color::get_colors(image.to_rgb8().into_raw().as_slice(), false);
    assert_eq!(colors.len(), 5 * 3);
    assert_eq!(
        colors,
        vec!(232, 230, 228, 58, 58, 10, 204, 52, 25, 191, 178, 56, 104, 152, 12)
    );
}

#[test]
fn image_with_alpha() {
    let image = image::open(path::Path::new("docs/Fotolia_45549559_320_480.jpg")).unwrap();
    let colors = dominant_color::get_colors(image.to_rgba8().into_raw().as_slice(), true);
    assert_eq!(colors.len(), 5 * 4);
    assert_eq!(
        colors,
        vec!(
            232, 230, 228, 255, 58, 58, 10, 255, 204, 52, 25, 255, 191, 178, 56, 255, 104, 152, 12,
            255
        )
    );
}
