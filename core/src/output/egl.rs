use super::OutputMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EglConfig {
    pub red_bits: u8,
    pub green_bits: u8,
    pub blue_bits: u8,
    pub alpha_bits: u8,
    pub depth_bits: u8,
    pub samples: u8,
}

impl Default for EglConfig {
    fn default() -> Self {
        EglConfig {
            red_bits: 8,
            green_bits: 8,
            blue_bits: 8,
            alpha_bits: 8,
            depth_bits: 24,
            samples: 0,
        }
    }
}

impl EglConfig {
    pub fn color_bits(&self) -> u8 {
        self.red_bits + self.green_bits + self.blue_bits + self.alpha_bits
    }

    pub fn is_suitable(&self) -> bool {
        self.red_bits + self.green_bits + self.blue_bits >= 24 && self.alpha_bits >= 8
    }
}

pub fn pick_config(configs: &[EglConfig]) -> Option<EglConfig> {
    configs
        .iter()
        .copied()
        .filter(|c| c.is_suitable())
        .max_by_key(|c| (c.color_bits(), c.samples))
}

pub fn swap_interval_range(vsync: bool) -> (i32, i32) {
    if vsync {
        (1, 1)
    } else {
        (0, 0)
    }
}

pub fn best_mode(modes: &[OutputMode], target_width: u32, target_height: u32) -> Option<OutputMode> {
    modes
        .iter()
        .copied()
        .min_by_key(|m| {
            let dx = (m.width as i128 - target_width as i128).abs();
            let dy = (m.height as i128 - target_height as i128).abs();
            (dx + dy).abs()
        })
}

pub fn modes_for_width(modes: &[OutputMode], width: u32) -> Vec<&OutputMode> {
    modes.iter().filter(|m| m.width == width).collect()
}