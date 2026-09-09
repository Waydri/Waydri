use super::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    pub fn from_points(a: Vec2, b: Vec2) -> Self {
        let min_x = a.x.min(b.x) as i32;
        let min_y = a.y.min(b.y) as i32;
        let max_x = a.x.max(b.x) as i32;
        let max_y = a.y.max(b.y) as i32;
        Rect::new(min_x, min_y, max_x - min_x, max_y - min_y)
    }

    pub fn is_empty(&self) -> bool {
        self.width <= 0 || self.height <= 0
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= self.x
            && y >= self.y
            && x < self.x + self.width
            && y < self.y + self.height
    }

    pub fn contains_point(&self, p: Vec2) -> bool {
        self.contains(p.x as i32, p.y as i32)
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        let x = self.x.max(other.x);
        let y = self.y.max(other.y);
        let right = (self.x + self.width).min(other.x + other.width);
        let bottom = (self.y + self.height).min(other.y + other.height);
        if right <= x || bottom <= y {
            None
        } else {
            Some(Rect::new(x, y, right - x, bottom - y))
        }
    }

    pub fn union(&self, other: &Rect) -> Rect {
        let x = self.x.min(other.x);
        let y = self.y.min(other.y);
        let right = (self.x + self.width).max(other.x + other.width);
        let bottom = (self.y + self.height).max(other.y + other.height);
        Rect::new(x, y, right - x, bottom - y)
    }

    pub fn translate(&self, dx: i32, dy: i32) -> Rect {
        Rect::new(self.x + dx, self.y + dy, self.width, self.height)
    }

    pub fn translated(&self, v: Vec2) -> Rect {
        self.translate(v.x as i32, v.y as i32)
    }

    pub fn grow(&self, amount: i32) -> Rect {
        Rect::new(
            self.x - amount,
            self.y - amount,
            self.width + amount * 2,
            self.height + amount * 2,
        )
    }

    pub fn shrink(&self, amount: i32) -> Rect {
        self.grow(-amount)
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.x as f32 + self.width as f32 / 2.0,
            self.y as f32 + self.height as f32 / 2.0,
        )
    }

    pub fn top_left(&self) -> Vec2 {
        Vec2::new(self.x as f32, self.y as f32)
    }

    pub fn area(&self) -> i64 {
        self.width as i64 * self.height as i64
    }

    pub fn clamped_to(&self, bounds: &Rect) -> Rect {
        let x = self.x.max(bounds.x);
        let y = self.y.max(bounds.y);
        let right = (self.x + self.width).min(bounds.x + bounds.width);
        let bottom = (self.y + self.height).min(bounds.y + bounds.height);
        Rect::new(x, y, (right - x).max(0), (bottom - y).max(0))
    }
}
