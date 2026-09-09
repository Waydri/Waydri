pub fn multiply_alpha(pixels: &mut [u8], alpha: f32) {
    let alpha = alpha.clamp(0.0, 1.0);
    if alpha >= 1.0 {
        return;
    }
    for pixel in pixels.chunks_exact_mut(4) {
        pixel[3] = (pixel[3] as f32 * alpha) as u8;
    }
}

pub fn modulate_rgba(pixels: &mut [u8], factor: f32, channel: usize) {
    let factor = factor.clamp(0.0, 255.0);
    for pixel in pixels.chunks_exact_mut(4) {
        pixel[channel] = (pixel[channel] as f32 * factor / 255.0) as u8;
    }
}

pub fn dim(pixels: &mut [u8], amount: f32) {
    let factor = (1.0 - amount.clamp(0.0, 1.0)) * 255.0;
    for pixel in pixels.chunks_exact_mut(4) {
        pixel[0] = (pixel[0] as f32 * factor / 255.0) as u8;
        pixel[1] = (pixel[1] as f32 * factor / 255.0) as u8;
        pixel[2] = (pixel[2] as f32 * factor / 255.0) as u8;
    }
}