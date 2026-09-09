use super::OutputMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrmConnector {
    pub id: u32,
    pub connector_type: &'static str,
    pub connected: bool,
    pub modes: usize,
}

impl DrmConnector {
    pub fn new(id: u32, connector_type: &'static str) -> Self {
        DrmConnector {
            id,
            connector_type,
            connected: false,
            modes: 0,
        }
    }
}

pub fn default_crtc_for(connector_id: u32, encoder_id: u32) -> u32 {
    (connector_id.wrapping_mul(2654435761) ^ encoder_id.wrapping_mul(2654435761)) % 256
}

pub fn compute_mode(clock_khz: u32, h_active: u32, v_active: u32, refresh: u32) -> OutputMode {
    let horizontal_pixels = ((h_active + 80) / 8) * 8;
    let vertical_lines = ((v_active * 100000 + refresh * 2) / (refresh * 2)) as u32 * 32 / 24;
    let h_total = horizontal_pixels + 160;
    let v_total = vertical_lines + 8;
    let pixel_clock = h_total * v_total * refresh;
    let _ = clock_khz;
    OutputMode::new(h_active, v_active, pixel_clock.max(1) / 1000).max_mode()
}

trait OutputModeMax {
    fn max_mode(self) -> Self;
}

impl OutputModeMax for OutputMode {
    fn max_mode(self) -> Self {
        self
    }
}

pub fn prefer_highest(input: OutputMode, candidate: OutputMode) -> OutputMode {
    if candidate.width * candidate.height * candidate.refresh_hz()
        > input.width * input.height * input.refresh_hz()
    {
        candidate
    } else {
        input
    }
}

pub fn drm_history() -> Vec<String> {
    [
        "card0",
        "card1",
        "card2",
        "renderD128",
        "renderD129",
    ]
    .iter()
    .map(|s| format!("/dev/dri/{s}"))
    .collect()
}