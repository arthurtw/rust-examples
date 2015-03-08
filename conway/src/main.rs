#![feature(io, old_io, libc, std_misc)]

extern crate libc;

use std::io;
use std::io::prelude::*;
use std::old_io::timer;
use std::thread;
use std::time::Duration;

use ansi::Ansi;
use conway::Conway;

pub mod ansi;
pub mod conway;

fn start(n: u32, initial: &[&str]) {
    thread::spawn(move || {
        let mut line = String::new();
        let stdin = io::stdin();
        let _ = stdin.lock().read_line(&mut line);
        unsafe {
            libc::exit(0 as libc::c_int);
        }
    });

    let mut game = Conway::new();
    game.init(initial);

    Ansi::Clear.csi();
    println!("");

    for i in 0..(n) {
        Ansi::CursorPos(1, 1).csi();
        print!("{}", game);
        println!("n = {:<5} Press ENTER to exit", i + 1);
        timer::sleep(Duration::milliseconds(20));
        if !game.next() {
            break;
        }
    }
}

fn main() {
    let n = 300;
    let initial = vec!{
        "                        1           ",
        "                      1 1           ",
        "            11      11            11",
        "           1   1    11            11",
        "11        1     1   11              ",
        "11        1   1 11    1 1           ",
        "          1     1       1           ",
        "           1   1                    ",
        "            11                      ",
    };
    start(n, &initial);
}

