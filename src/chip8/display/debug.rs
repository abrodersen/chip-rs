
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
    fn clear(&mut self) {
        println!("display clear!");
    }

    fn draw(&mut self, pt: Point, sprite: &Sprite) {
        let (max_x, over_x) = pt.x.overflowing_add(8);
        let (max_y, over_y) = pt.y.overflowing_add(sprite.raw().len() as u8);

        if over_x || over_y {
            panic!("WARN: Overflow addition! ({}, {})", pt.x, pt.y);
        }

        println!("drawing sprite[{}] from ({}, {}) to ({}, {})", sprite.buffer.len(), pt.x, pt.y, max_x, max_y);
        if max_x > 64 || max_y > 32 {
            println!("WARN: Overflowing write!");
        }
    }
}