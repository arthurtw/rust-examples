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
        let seq = match *self {
            Ansi::Clear => "2J".to_string(),
            Ansi::CursorUp(n) => format!("{}A", n),
            Ansi::CursorDown(n) => format!("{}B", n),
            Ansi::CursorForward(n) => format!("{}C", n),
            Ansi::CursorBack(n) => format!("{}D", n),
            Ansi::CursorPos(row, col) => format!("{};{}H", row, col),
            Ansi::EraseToEOL => "K".to_string(),
        };
        print!("\x1b[{}", seq);
    }
}

