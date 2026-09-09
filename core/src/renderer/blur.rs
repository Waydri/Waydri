pub fn gaussian_kernel(radius: usize) -> Vec<f32> {
    let radius = radius.clamp(1, 64);
    let sigma = radius as f32 / 3.0;
    let mut kernel = Vec::with_capacity(radius * 2 + 1);
    let center = radius as f32;
    let mut sum = 0.0;
    for i in 0..=radius * 2 {
        let distance = (i as f32 - center).abs();
        let weight = (-(distance * distance) / (2.0 * sigma * sigma)).exp();
        kernel.push(weight);
        sum += weight;
    }
    for weight in kernel.iter_mut() {
        *weight /= sum;
    }
    kernel
}

pub fn blur_horizontal(pixels: &mut [u8], width: usize, height: usize, kernel: &[f32]) {
    let radius = kernel.len() / 2;
    let mut row = Vec::with_capacity(width * 4);
    for y in 0..height {
        row.clear();
        for x in 0..width {
            let mut accum = [0.0f32; 4];
            for (k, weight) in kernel.iter().enumerate() {
                let sx = x as isize + k as isize - radius as isize;
                let sx = sx.clamp(0, width as isize - 1) as usize;
                let index = (y * width + sx) * 4;
                for c in 0..4 {
                    accum[c] += pixels[index + c] as f32 * weight;
                }
            }
            for c in 0..4 {
                row.push(accum[c] as u8);
            }
        }
        let start = y * width * 4;
        pixels[start..start + width * 4].copy_from_slice(&row);
    }
}

pub fn blur_vertical(pixels: &mut [u8], width: usize, height: usize, kernel: &[f32]) {
    let radius = kernel.len() / 2;
    let mut column = Vec::with_capacity(height * 4);
    for x in 0..width {
        column.clear();
        for y in 0..height {
            let mut accum = [0.0f32; 4];
            for (k, weight) in kernel.iter().enumerate() {
                let sy = y as isize + k as isize - radius as isize;
                let sy = sy.clamp(0, height as isize - 1) as usize;
                let index = (sy * width + x) * 4;
                for c in 0..4 {
                    accum[c] += pixels[index + c] as f32 * weight;
                }
            }
            for c in 0..4 {
                column.push(accum[c] as u8);
            }
        }
        for (y, value) in column.chunks(4).enumerate() {
            let index = (y * width + x) * 4;
            pixels[index..index + 4].copy_from_slice(value);
        }
    }
}

pub fn blur(pixels: &mut [u8], width: usize, height: usize, radius: usize) {
    if width == 0 || height == 0 || radius == 0 {
        return;
    }
    let kernel = gaussian_kernel(radius);
    blur_horizontal(pixels, width, height, &kernel);
    blur_vertical(pixels, width, height, &kernel);
}

pub fn blur_premultiplied_region(
    source: &[u8],
    dst: &mut [u8],
    width: usize,
    _height: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    radius: usize,
) {
    let mut region: Vec<u8> = source
        .chunks_exact(width * 4)
        .skip(y)
        .take(h)
        .flat_map(|row| row[x * 4..(x + w) * 4].iter().copied())
        .collect();
    blur(&mut region, w, h, radius);
    for sy in 0..h {
        for sx in 0..w {
            let src_index = ((y + sy) * width + x + sx) * 4;
            let reg_index = (sy * w + sx) * 4;
            dst[src_index..src_index + 4].copy_from_slice(&region[reg_index..reg_index + 4]);
        }
    }
}