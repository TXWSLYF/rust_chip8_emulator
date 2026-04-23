use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct Display {
    display: [[bool; WINDOW_WIDTH]; WINDOW_HEIGHT],
}

impl Display {
    pub fn new() -> Self {
        Self {
            display: [[false; WINDOW_WIDTH]; WINDOW_HEIGHT],
        }
    }
}

impl Display {
    pub fn clear(&mut self) {
        self.display = [[false; WINDOW_WIDTH]; WINDOW_HEIGHT];
    }
}
