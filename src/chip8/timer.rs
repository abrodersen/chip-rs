
extern crate time;

pub struct Timer {
    timestamp: u64,
    time: u8,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            time: 0,
            timestamp: 0
        }
    }

    pub fn get (&mut self) -> u8 {
        self.time
    }

    pub fn tick(&mut self) {
        if self.time == 0 {
            return;
        }

        let next = time::precise_time_ns();
        if next - self.timestamp > 1_666_666 {
            self.time -= 1;
        }

        if self.time == 0 {
            println!("timer hit zero!");
        }
    }

    pub fn set(&mut self, time: u8) {
        println!("timer updated to {}", time);
        self.timestamp = time::precise_time_ns();
        self.time = time;
    }
}