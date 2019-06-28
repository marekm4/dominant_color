#[derive(PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn get_colors(pixels: &[u8]) -> Vec<Color> {
    get_colors_with_config(&pixels, 224.0, 0.01)
}

pub fn get_colors_with_config(pixels: &[u8], down_size_to: f64, small_bucket: f64) -> Vec<Color> {
    let mut colors: Vec<Color> = Vec::new();

    if pixels.len() >= 3 {
        colors.push(Color {
            r: pixels[0],
            g: pixels[1],
            b: pixels[2],
        });
    }

    colors
}
