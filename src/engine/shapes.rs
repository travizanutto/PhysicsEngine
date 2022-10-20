use super::physics::*;

pub struct Rectangle {
    pub x: isize,
    pub y: isize,
    pub width: usize,
    pub height: usize
}

impl Rectangle {
    pub fn new(size: (usize, usize)) -> Rectangle {
        Rectangle { x: 0, y: 0, width: size.0, height: size.1 }
    }
}

pub struct Hitbox {
    rect: Option<Rectangle>,
    vel: Option<Vec2>
}

impl Default for Hitbox {
    fn default() -> Self {
        Hitbox { rect: None, vel: None }
    }
}