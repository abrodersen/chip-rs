
extern crate sdl2;

use super::Input;

use sdl2::keyboard::Keycode;
use sdl2::event::Event;

pub struct SdlInput<'a> {
    events: &'a mut sdl2::EventPump
}

impl<'a> SdlInput<'a> {
    pub fn new(events: &mut sdl2::EventPump) -> SdlInput {
        SdlInput {
            events: events
        }
    }
}

fn key_map(key: u8) -> Keycode {
    match key {
        0x0 => Keycode::X,
        0x1 => Keycode::Num1,
        0x2 => Keycode::Num2,
        0x3 => Keycode::Num3,
        0x4 => Keycode::Q,
        0x5 => Keycode::W,
        0x6 => Keycode::E,
        0x7 => Keycode::A,
        0x8 => Keycode::S,
        0x9 => Keycode::D,
        0xA => Keycode::Z,
        0xB => Keycode::C,
        0xC => Keycode::Num4,
        0xD => Keycode::R,
        0xE => Keycode::F,
        0xF => Keycode::V,
        _ => panic!("invalid key code: {}", key),
    }
}

impl<'a> Input for SdlInput<'a> {
    fn is_pressed(&mut self, key: u8) -> bool {
        self.events.keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .any(|x| x == key_map(key))
    }

    fn wait_pressed(&mut self, key: u8) {
        loop {
            let next = self.events.wait_event();
            match next {
                Event::KeyDown{ keycode: Some(k), timestamp:_, window_id: _, scancode:_, keymod:_, repeat:_ } => {
                    if k == key_map(key) {
                        return;
                    }
                }
                _ => { }
            };
        }
    }
}