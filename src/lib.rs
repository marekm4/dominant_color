#[derive(PartialEq, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Copy, Clone, Debug)]
struct Bucket {
    r: f64,
    g: f64,
    b: f64,
    count: f64,
}

pub fn get_colors(pixels: &[u8]) -> Vec<Color> {
    get_colors_with_config(&pixels, 224.0 * 224.0, 0.01)
}

pub fn get_colors_with_config(pixels: &[u8], down_size_to: f64, small_bucket: f64) -> Vec<Color> {
    // calculate down size step
    let pixel_count = pixels.len() / 3;
    let step = (pixel_count / down_size_to.round() as usize).max(1);

    // load pixels to buckets
    let mut buckets = [[[Bucket {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        count: 0.0,
    }; 2]; 2]; 2];
    for i in (0..pixels.len()).step_by(3 * step) {
        let r = pixels[i];
        let g = pixels[i + 1];
        let b = pixels[i + 2];
        let i = (r >> 7) as usize;
        let j = (g >> 7) as usize;
        let k = (b >> 7) as usize;
        buckets[i][j][k].r += r as f64;
        buckets[i][j][k].g += g as f64;
        buckets[i][j][k].b += b as f64;
        buckets[i][j][k].count += 1.0;
    }

    // calculate buckets averages
    let mut buckets_averages: Vec<Bucket> = Vec::new();
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let current_bucket = &buckets[i][j][k];
                if current_bucket.count > 0.0 {
                    buckets_averages.push(Bucket {
                        r: current_bucket.r / current_bucket.count,
                        g: current_bucket.g / current_bucket.count,
                        b: current_bucket.b / current_bucket.count,
                        count: current_bucket.count,
                    })
                }
            }
        }
    }

    // sort buckets averages
    buckets_averages.sort_by(|a, b| b.count.partial_cmp(&a.count).unwrap());

    // convert buckets to colors, ignore small buckets
    let mut colors: Vec<Color> = Vec::new();
    let sampled_pixel_count = pixel_count / step;
    for ba in &buckets_averages {
        if ba.count / sampled_pixel_count as f64 > small_bucket {
            colors.push(Color {
                r: ba.r.round() as u8,
                g: ba.g.round() as u8,
                b: ba.b.round() as u8,
            });
        }
    }

    colors
}
