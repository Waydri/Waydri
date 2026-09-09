use crate::utils::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Left,
    Middle,
    Right,
    Back,
    Forward,
    Side,
    Extra,
}

impl Button {
    pub fn from_code(code: u32) -> Option<Button> {
        match code {
            0x110 => Some(Button::Left),
            0x111 => Some(Button::Middle),
            0x112 => Some(Button::Right),
            0x113 => Some(Button::Back),
            0x114 => Some(Button::Forward),
            0x115 => Some(Button::Side),
            0x116 => Some(Button::Extra),
            _ => None,
        }
    }

    pub fn code(self) -> u32 {
        match self {
            Button::Left => 0x110,
            Button::Middle => 0x111,
            Button::Right => 0x112,
            Button::Back => 0x113,
            Button::Forward => 0x114,
            Button::Side => 0x115,
            Button::Extra => 0x116,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollAxis {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
pub struct Pointer {
    pub position: Vec2,
    pub buttons: Vec<Button>,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub motion_x: f32,
    pub motion_y: f32,
}

impl Default for Pointer {
    fn default() -> Self {
        Self::new()
    }
}

impl Pointer {
    pub fn new() -> Self {
        Pointer {
            position: Vec2::ZERO,
            buttons: Vec::new(),
            scroll_x: 0.0,
            scroll_y: 0.0,
            motion_x: 0.0,
            motion_y: 0.0,
        }
    }

    pub fn move_to(&mut self, position: Vec2) {
        self.motion_x += position.x - self.position.x;
        self.motion_y += position.y - self.position.y;
        self.position = position;
    }

    pub fn move_by(&mut self, dx: f32, dy: f32) {
        self.position.x += dx;
        self.position.y += dy;
        self.motion_x += dx;
        self.motion_y += dy;
    }

    pub fn press(&mut self, button: Button) -> bool {
        if !self.buttons.contains(&button) {
            self.buttons.push(button);
            true
        } else {
            false
        }
    }

    pub fn release(&mut self, button: Button) -> bool {
        let before = self.buttons.len();
        self.buttons.retain(|b| *b != button);
        self.buttons.len() != before
    }

    pub fn is_pressed(&self, button: Button) -> bool {
        self.buttons.contains(&button)
    }

    pub fn scroll(&mut self, axis: ScrollAxis, amount: f32) {
        match axis {
            ScrollAxis::Vertical => self.scroll_y += amount,
            ScrollAxis::Horizontal => self.scroll_x += amount,
        }
    }

    pub fn take_motion(&mut self) -> Vec2 {
        let motion = Vec2::new(self.motion_x, self.motion_y);
        self.motion_x = 0.0;
        self.motion_y = 0.0;
        motion
    }

    pub fn take_scroll_y(&mut self) -> f32 {
        let value = self.scroll_y;
        self.scroll_y = 0.0;
        value
    }

    pub fn take_scroll_x(&mut self) -> f32 {
        let value = self.scroll_x;
        self.scroll_x = 0.0;
        value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonAction {
    Press,
    Release,
}

#[derive(Debug, Clone, Copy)]
pub struct PointerEvent {
    pub button: Option<Button>,
    pub action: Option<ButtonAction>,
    pub position: Vec2,
    pub scroll: Option<(ScrollAxis, f32)>,
}