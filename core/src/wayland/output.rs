use crate::output::OutputManager;
use crate::utils::{Rect, Transform};

#[derive(Debug, Clone)]
pub struct WlOutput {
    pub name: String,
    pub global_id: u32,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub refresh: u32,
    pub transform: Transform,
    pub scale: i32,
    pub enabled: bool,
}

impl WlOutput {
    pub fn from_headless(name: String, global_id: u32, width: i32, height: i32, refresh: u32) -> Self {
        WlOutput {
            name,
            global_id,
            x: 0,
            y: 0,
            width,
            height,
            refresh,
            transform: Transform::Normal,
            scale: 1,
            enabled: true,
        }
    }

    pub fn geometry(&self) -> Rect {
        let (w, h) = self.transform.apply(self.width, self.height);
        Rect::new(self.x, self.y, w, h)
    }

    pub fn from_manager(manager: &OutputManager) -> Vec<WlOutput> {
        manager
            .clone_geometries()
            .iter()
            .enumerate()
            .map(|(index, rect)| {
                WlOutput::from_headless(
                    format!("output {}", index + 1),
                    (index + 1) as u32,
                    rect.width,
                    rect.height,
                    60000,
                )
            })
            .collect()
    }

    pub fn mhz(&self) -> u32 {
        self.refresh
    }
}

pub fn parse_output_line(line: &str) -> Option<(String, i32, i32, u32)> {
    let mut parts = line.split_whitespace();
    let name = parts.next()?.to_string();
    let width = parts.next()?.parse().ok()?;
    let height = parts.next()?.parse().ok()?;
    let refresh = parts.next().and_then(|v| v.parse().ok()).unwrap_or(60000);
    Some((name, width, height, refresh))
}

pub fn global_for_output(id: u32, name: &str) -> String {
    format!("{}:{}", name, id)
}