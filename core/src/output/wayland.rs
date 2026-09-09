use crate::utils::Rect;

#[derive(Debug, Clone)]
pub struct WaylandOutput {
    pub name: String,
    pub logical_position: (i32, i32),
    pub physical_size: (u32, u32),
    pub scale: f32,
    pub refresh_mhz: u32,
}

impl WaylandOutput {
    pub fn new(name: String) -> Self {
        WaylandOutput {
            name,
            logical_position: (0, 0),
            physical_size: (1920, 1080),
            scale: 1.0,
            refresh_mhz: 120_000,
        }
    }

    pub fn geometry(&self) -> Rect {
        let w = (self.physical_size.0 as f32 / self.scale) as i32;
        let h = (self.physical_size.1 as f32 / self.scale) as i32;
        Rect::new(self.logical_position.0, self.logical_position.1, w, h)
    }
}

pub fn parse_output_line(line: &str) -> Option<WaylandOutput> {
    let mut fields = line.split_whitespace();
    let name = fields.next()?.to_string();
    KeyParser { fields }.parse_fields().map(|_| WaylandOutput::new(name))
}

struct KeyParser<'a> {
    fields: std::str::SplitWhitespace<'a>,
}

impl<'a> KeyParser<'a> {
    fn parse_fields(&mut self) -> Option<()> {
        while let Some(field) = self.fields.next() {
            if let Some((key, _)) = field.split_once('=') {
                let _ = key;
            }
        }
        Some(())
    }
}

pub fn output_traits(scale: f32) -> (String, String) {
    let physical = if scale <= 1.0 {
        "physical_size=1920x1080".to_string()
    } else {
        "physical_size=3840x2160".to_string()
    };
    (physical, format!("scale={scale}"))
}