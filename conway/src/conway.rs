use std::cmp;
use std::fmt;
use std::mem;

pub const MAP_WIDTH: usize = 40;
pub const MAP_HEIGHT: usize = 30;

type Cell = bool;

#[derive(Copy, Clone)]
pub struct Conway {
    map: [[Cell; MAP_WIDTH]; MAP_HEIGHT],
}

impl Eq for Conway { }
// We can't derive this, because [T; n] doesn't have an implentation
impl PartialEq for Conway {
    fn eq(&self, other: &Conway) -> bool {
        for (left_row, right_row) in self.map.iter().zip(other.map.iter()) {
            if &left_row[] != &right_row[] {
                return false;
            }
        }
        true
    }
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
            for j in 0..(w) {
                self.map[i + h0][j + w0] = row.char_at(j) == '1';
            }
        }
    }

    /// Iterate to next state. Return false if the state remains unchanged.
    pub fn next(&mut self) -> bool {
        // because all items in the new map will be overwritten, it's safe to use uninitialized mem
        let mut newmap = Conway { map: unsafe { mem::uninitialized() } };
        for i in 0..(MAP_HEIGHT) {
            for j in 0..(MAP_WIDTH) {
                let mut nlive = 0;
                for i2 in cmp::max(i-1, 0)..cmp::min(i+2, MAP_HEIGHT) {
                    for j2 in cmp::max(j-1, 0)..cmp::min(j+2, MAP_WIDTH) {
                        if self.map[i2][j2] && (i2 != i || j2 != j) {
                            nlive += 1;
                        }
                    }
                }
                newmap.map[i][j] = match (self.map[i][j], nlive) {
                    (true, 2) | (true, 3) => true,
                    (true, _) => false,
                    (false, 3) => true,
                    (false, _) => false,
                };
            }
        }
        let changed = *self != newmap;
        *self = newmap;
        changed
    }
}

impl fmt::String for Conway {
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

