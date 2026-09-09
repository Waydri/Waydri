#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    Background,
    Bottom,
    Top,
    Overlay,
}

impl Layer {
    pub fn from_layer(layer: u32) -> Layer {
        match layer {
            0 => Layer::Background,
            1 => Layer::Bottom,
            2 => Layer::Top,
            _ => Layer::Overlay,
        }
    }

    pub fn z_index(&self) -> u32 {
        match self {
            Layer::Background => 0,
            Layer::Bottom => 100,
            Layer::Top => 200,
            Layer::Overlay => 300,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayerSurface {
    pub surface_id: u32,
    pub namespace: String,
    pub layer: Layer,
    pub anchors: u32,
    pub margin: (i32, i32, i32, i32),
    pub exclusive_zone: i32,
    pub width: u32,
    pub height: u32,
}

impl LayerSurface {
    pub fn new(surface_id: u32, layer: Layer, namespace: String) -> Self {
        LayerSurface {
            surface_id,
            namespace,
            layer,
            anchors: 0,
            margin: (0, 0, 0, 0),
            exclusive_zone: 0,
            width: 0,
            height: 0,
        }
    }

    pub fn set_anchors(&mut self, anchors: u32) {
        self.anchors = anchors;
    }

    pub fn is_top(&self, anchor_bit: u32) -> bool {
        self.anchors & anchor_bit != 0
    }
}

pub const ANCHOR_TOP: u32 = 1;
pub const ANCHOR_BOTTOM: u32 = 2;
pub const ANCHOR_LEFT: u32 = 4;
pub const ANCHOR_RIGHT: u32 = 8;

pub fn anchor_name(anchors: u32) -> String {
    let mut parts = Vec::new();
    if anchors & ANCHOR_TOP != 0 {
        parts.push("top");
    }
    if anchors & ANCHOR_BOTTOM != 0 {
        parts.push("bottom");
    }
    if anchors & ANCHOR_LEFT != 0 {
        parts.push("left");
    }
    if anchors & ANCHOR_RIGHT != 0 {
        parts.push("right");
    }
    if parts.is_empty() {
        "none".to_string()
    } else {
        parts.join(" ")
    }
}