
pub struct Timer {
    time: u8
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            time: 0
        }
    }

    pub fn get (&mut self) -> u8 {
        self.time
    }

    pub fn set(&mut self, time: u8) {
        println!("timer updated to {}", time);
        self.time = time;
    }
}