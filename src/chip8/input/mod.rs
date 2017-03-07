
pub mod sdl;

pub trait Input {
    fn is_pressed(&mut self, u8) -> bool;
    fn wait_pressed(&mut self, u8);
}