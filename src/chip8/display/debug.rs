
use super::{Display, Point, Sprite};

pub struct DebugDisplay {

}

impl DebugDisplay {
    pub fn new() -> DebugDisplay {
        DebugDisplay {

        }
    }
}

impl Display for DebugDisplay {
    fn draw(&mut self, pt: Point, sprite: &Sprite) {
        println!("drawing sprite[{}] at ({}, {})", sprite.buffer.len(), pt.x, pt.y);
    }
}