use super::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transform {
    Normal,
    Rotated90,
    Rotated180,
    Rotated270,
    Flipped,
    FlippedRotated90,
    FlippedRotated180,
    FlippedRotated270,
}

impl Transform {
    pub fn from_degrees(degrees: i32) -> Self {
        match degrees.rem_euclid(360) {
            0 => Transform::Normal,
            90 => Transform::Rotated90,
            180 => Transform::Rotated180,
            270 => Transform::Rotated270,
            _ => Transform::Normal,
        }
    }

    pub fn degrees(&self) -> i32 {
        match self {
            Transform::Normal | Transform::Flipped => 0,
            Transform::Rotated90 | Transform::FlippedRotated90 => 90,
            Transform::Rotated180 | Transform::FlippedRotated180 => 180,
            Transform::Rotated270 | Transform::FlippedRotated270 => 270,
        }
    }

    pub fn flipped(&self) -> bool {
        matches!(
            self,
            Transform::Flipped
                | Transform::FlippedRotated90
                | Transform::FlippedRotated180
                | Transform::FlippedRotated270
        )
    }

    pub fn apply(&self, width: i32, height: i32) -> (i32, i32) {
        match self {
            Transform::Normal
            | Transform::Rotated180
            | Transform::Flipped
            | Transform::FlippedRotated180 => (width, height),
            Transform::Rotated90
            | Transform::Rotated270
            | Transform::FlippedRotated90
            | Transform::FlippedRotated270 => (height, width),
        }
    }

    pub fn transform_rect(&self, rect: Rect) -> Rect {
        let (w, h) = self.apply(rect.width, rect.height);
        Rect::new(rect.x, rect.y, w, h)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Transform::Normal
    }
}
