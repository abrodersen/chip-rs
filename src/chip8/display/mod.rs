
pub mod debug;
pub mod sdl;

pub struct Sprite {
    buffer: Vec<u8>,
}

pub struct Pixel {
    x: u8,
    y: u8,
    on: bool,
}

pub struct PixelIter<'a> {
    sprite: &'a Sprite,
    pos: u8,
}

lazy_static! {
    pub static ref ZERO:  Sprite = { Sprite::new(&[0xF0, 0x90, 0x90, 0x90, 0xF0]) };
    pub static ref ONE:   Sprite = { Sprite::new(&[0x20, 0x60, 0x20, 0x20, 0x70]) };
    pub static ref TWO:   Sprite = { Sprite::new(&[0xF0, 0x10, 0xF0, 0x80, 0xF0]) };
    pub static ref THREE: Sprite = { Sprite::new(&[0xF0, 0x10, 0xF0, 0x10, 0xF0]) };
    pub static ref FOUR:  Sprite = { Sprite::new(&[0x90, 0x90, 0xF0, 0x10, 0x10]) };
    pub static ref FIVE:  Sprite = { Sprite::new(&[0xF0, 0x80, 0xF0, 0x10, 0xF0]) };
    pub static ref SIX:   Sprite = { Sprite::new(&[0xF0, 0x80, 0xF0, 0x90, 0xF0]) };
    pub static ref SEVEN: Sprite = { Sprite::new(&[0xF0, 0x10, 0x20, 0x40, 0x40]) };
    pub static ref EIGHT: Sprite = { Sprite::new(&[0xF0, 0x90, 0xF0, 0x90, 0xF0]) };
    pub static ref NINE:  Sprite = { Sprite::new(&[0xF0, 0x90, 0xF0, 0x10, 0xF0]) };
    pub static ref A:     Sprite = { Sprite::new(&[0xF0, 0x90, 0xF0, 0x90, 0x90]) };
    pub static ref B:     Sprite = { Sprite::new(&[0xE0, 0x90, 0xE0, 0x90, 0xE0]) };
    pub static ref C:     Sprite = { Sprite::new(&[0xF0, 0x80, 0x80, 0x80, 0xF0]) };
    pub static ref D:     Sprite = { Sprite::new(&[0xE0, 0x90, 0x90, 0x90, 0xE0]) };
    pub static ref E:     Sprite = { Sprite::new(&[0xF0, 0x80, 0xF0, 0x80, 0xF0]) };
    pub static ref F:     Sprite = { Sprite::new(&[0xF0, 0x80, 0xF0, 0x80, 0x80]) };
}

impl Sprite {
     pub fn new(data: &[u8]) -> Sprite {
         Sprite {
             buffer: Vec::from(data),
         }
     }

     pub fn pixels(&self) -> PixelIter {
        PixelIter {
            sprite: self,
            pos: 0,
        }
     }

     pub fn raw(&self) -> &[u8] {
        self.buffer.as_slice()
     }
}

impl<'a> Iterator for PixelIter<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        let byte = self.pos / 8;
        if byte >= self.sprite.buffer.len() as u8 {
            return None;
        }

        let value = self.sprite.buffer[byte as usize];
        let bit = self.pos % 8;
        //println!("byte {} bit {}", byte, bit);
        let on = (value >> (7 - bit)) & 0x1 == 1;
        self.pos += 1;

        Some(Pixel{ x: byte, y: bit, on: on})
    }
}

pub struct Point {
    pub x: u8,
    pub y: u8,
}

pub trait Display {
     fn draw(&mut self, pt: Point, sp: &Sprite);
}

