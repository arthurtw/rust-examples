use std::cmp;
use std::fmt;
use std::num::Int;

pub const MAP_WIDTH: usize = 40;
pub const MAP_HEIGHT: usize = 30;

type Cell = bool;

#[derive(Copy)]
pub struct Conway {
    map: [[Cell; MAP_WIDTH]; MAP_HEIGHT],
}

impl Conway {
    pub fn new() -> Conway {
        Conway {
            map: [[false; MAP_WIDTH]; MAP_HEIGHT],
        }
    }

    pub fn init(&mut self, pattern: &[&str]) {
        let h = pattern.len();
        let h0 = (MAP_HEIGHT - h) / 2;
        for i in 0..(h) {
            let row = pattern[i];
            let w = row.len();
            let w0 = (MAP_WIDTH - w) / 2;
            for (j, c) in row.chars().enumerate() {
                self.map[i + h0][j + w0] = c == '1';
            }
        }
    }

    /// Iterate to next state. Return false if the state remains unchanged.
    pub fn next(&mut self) -> bool {
        let mut newmap = [[false; MAP_WIDTH]; MAP_HEIGHT];
        for i in 0..(MAP_HEIGHT) {
            for j in 0..(MAP_WIDTH) {
                let mut nlive = 0;
                for i2 in i.saturating_sub(1)..cmp::min(i+2, MAP_HEIGHT) {
                    for j2 in j.saturating_sub(1)..cmp::min(j+2, MAP_WIDTH) {
                        if self.map[i2][j2] && (i2 != i || j2 != j) {
                            nlive += 1;
                        }
                    }
                }
                newmap[i][j] = match (self.map[i][j], nlive) {
                    (true, 2) | (true, 3) => true,
                    (true, _) => false,
                    (false, 3) => true,
                    (false, _) => false,
                };
            }
        }
        // let changed = self.map != newmap;
        let changed = true;
        self.map = newmap;
        changed
    }
}

impl fmt::Display for Conway {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.map.iter() {
            for cell in row.iter() {
                try!(write!(f, "{}", if *cell { "()" } else { ". " }));
            }
            try!(write!(f, "\n"));
        }
        Ok(())
    }
}

