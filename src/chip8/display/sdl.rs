
use super::{Display, Pixel, Point, Sprite};

use sdl2;
use sdl2::pixels::Color;
use sdl2::rect;

use std::iter::*;
use std::convert::From;

pub struct SdlDisplay<'a> {
    renderer: sdl2::render::Renderer<'a>,
    state: Box<[bool]>,
}

const BLACK: Color = Color::RGB(0, 0, 0);
const WHITE: Color = Color::RGB(255, 255, 255);

const PIXEL_WIDTH : u32 = 64;
const PIXEL_HEIGHT : u32 = 32;
const SCALE_FACTOR : u32 = 24;

impl<'a> SdlDisplay<'a> {
    pub fn new(video: &sdl2::VideoSubsystem) -> SdlDisplay<'a> {
        let width = PIXEL_WIDTH * SCALE_FACTOR;
        let height = PIXEL_HEIGHT * SCALE_FACTOR;
        let window = video.window("CHIP-RS", width, height)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut renderer = window.renderer().build().unwrap();
        renderer.set_scale(SCALE_FACTOR as f32, SCALE_FACTOR as f32);

        let vram_size = (PIXEL_WIDTH * PIXEL_HEIGHT) as usize;
        let vram = vec![false; vram_size].into_boxed_slice();

        let mut disp = SdlDisplay {
            renderer: renderer,
            state: vram,
        };

        disp.clear();

        disp
    }
}

impl From<Pixel> for sdl2::rect::Point {
    fn from(pix: Pixel) -> sdl2::rect::Point {
        sdl2::rect::Point::new(pix.x as i32, pix.y as i32)
    }
}

impl<'a> Display for SdlDisplay<'a> {
    fn clear(&mut self) {
        self.renderer.set_draw_color(BLACK);
        self.renderer.clear();
        self.renderer.present();
    }

    fn draw(&mut self, pt: Point, sprite: &Sprite) {
        let pixels: Vec<_> = sprite.pixels()
            .map(|pix| {
                let x = (pt.x + pix.x) % PIXEL_WIDTH as u8;
                let y = (pt.y + pix.y) % PIXEL_HEIGHT as u8;
                let index = y as usize * PIXEL_WIDTH as usize + x as usize;
                let on = pix.on ^ self.state[index];
                self.state[index] = on;
                Pixel{ x: x, y: y, on: on }
            })
            .collect();

        let white: Vec<_> = pixels.clone().into_iter().filter(|pix| pix.on).map(|x| x.into()).collect();
        let black: Vec<_> = pixels.clone().into_iter().filter(|pix| !pix.on).map(|x| x.into()).collect();

        let ref mut r = self.renderer;

        r.set_draw_color(WHITE);
        r.draw_points(white.as_slice());

        r.set_draw_color(BLACK);
        r.draw_points(black.as_slice());

        r.present();
    }
}