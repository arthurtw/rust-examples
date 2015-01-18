#[allow(dead_code)]
#[derive(Copy)]
pub enum Ansi {
    Clear,
    CursorUp(u16),
    CursorDown(u16),
    CursorForward(u16),
    CursorBack(u16),
    CursorPos(u16, u16),
    EraseToEOL,
}

impl Ansi {
    pub fn csi(&self) {
        print!("\x1b[");
        match *self {
            Ansi::Clear => print!("2J"),
            Ansi::CursorUp(n) => print!("{}A", n),
            Ansi::CursorDown(n) => print!("{}B", n),
            Ansi::CursorForward(n) => print!("{}C", n),
            Ansi::CursorBack(n) => print!("{}D", n),
            Ansi::CursorPos(row, col) => print!("{};{}H", row, col),
            Ansi::EraseToEOL => print!("K"),
        };
    }
}

