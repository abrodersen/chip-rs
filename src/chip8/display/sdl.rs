
use super::{Display, Point, Sprite};

use sdl2;
use sdl2::pixels::Color;

use std::iter::FromIterator;

pub struct SdlDisplay<'a> {
    renderer: sdl2::render::Renderer<'a>
}

const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(255, 255, 255);

impl<'a> SdlDisplay<'a> {
    pub fn new(scale: u32, window: sdl2::video::Window) -> SdlDisplay<'a> {

        let mut renderer = window.renderer().build().unwrap();
        renderer.set_scale(scale as f32, scale as f32);
        renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.present();

        SdlDisplay {
            renderer: renderer,
        }
    }
}

impl<'a> Display for SdlDisplay<'a> {
    fn draw(&mut self, pt: Point, sprite: &Sprite) {
        let points: Vec<_> = sprite.pixels().filter(|pix| pix.on).map(|pix| {
            let x = pt.x + pix.x;
            let y = pt.y + pix.y;
            sdl2::rect::Point::new(x as i32, y as i32)
        }).collect();

        self.renderer.draw_points(points.as_slice());
        self.renderer.present();
    }
}