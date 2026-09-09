pub fn corner_alpha(x: usize, y: usize, radius: usize) -> f32 {
    if radius == 0 || x >= radius || y >= radius {
        return 1.0;
    }
    let dx = (radius - x - 1) as f32 + 0.5;
    let dy = (radius - y - 1) as f32 + 0.5;
    let distance = (dx * dx + dy * dy).sqrt();
    if distance <= radius as f32 - 0.5 {
        return 1.0;
    }
    if distance >= radius as f32 + 0.5 {
        return 0.0;
    }
    (radius as f32 + 0.5 - distance).clamp(0.0, 1.0)
}

pub fn apply_rounded_corners(pixels: &mut [u8], width: usize, height: usize, radius: usize) {
    if radius == 0 {
        return;
    }
    let mut index = 0;
    for y in 0..height {
        for x in 0..width {
            let alpha = if y < radius && x < radius {
                corner_alpha(x, y, radius)
            } else if y < radius && x >= width - radius {
                corner_alpha(width - x - 1, y, radius)
            } else if y >= height - radius && x < radius {
                corner_alpha(x, height - y - 1, radius)
            } else if y >= height - radius && x >= width - radius {
                corner_alpha(width - x - 1, height - y - 1, radius)
            } else {
                1.0
            };
            if alpha < 1.0 {
                pixels[index + 3] = (pixels[index + 3] as f32 * alpha) as u8;
            }
            index += 4;
        }
    }
}

pub fn mask_row(width: usize, radius: usize) -> Vec<u8> {
    let mut row = vec![255u8; width];
    if radius == 0 {
        return row;
    }
    for x in 0..radius.min(width) {
        let alpha = corner_alpha(x, 0, radius);
        row[x] = (alpha * 255.0) as u8;
        let right = width - 1 - x;
        if right < width {
            row[right] = (alpha * 255.0) as u8;
        }
    }
    row
}