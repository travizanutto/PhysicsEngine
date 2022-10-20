use super::shapes::*;

pub struct Plane {
    pub width: usize,
    pub height: usize,
    pub hitboxes: Vec<Rectangle>
}

impl Plane {
    pub fn new(size: (usize, usize)) -> Plane {
        Plane {
        width: size.0,
        height: size.1,
        hitboxes: Vec::new()
        }
    }
}

pub struct Vec2 {
    x: f64,
    y: f64
}