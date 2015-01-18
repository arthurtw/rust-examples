// Uncomment the following line to disable unstable warnings:
// #![allow(unstable)]

use std::io::{self, Timer};
use std::thread::Thread;
use std::time::Duration;
use std::sync::mpsc::channel;

use ansi::Ansi;
use conway::Conway;

pub mod ansi;
pub mod conway;

fn start(n: u32, initial: &[&str]) {
    let (tx, quit_thread) = channel();
    Thread::spawn(move || {
        let _ = io::stdin().read_line();
        let _ = tx.send(());
    });

    let mut game = Conway::new();
    game.init(initial);

    Ansi::Clear.csi();
    println!("");

    let mut timer = Timer::new().unwrap();
    let timer = timer.periodic(Duration::milliseconds(20));

    for i in 0..(n) {
        Ansi::CursorPos(1, 1).csi();
        print!("{}", game);
        println!("n = {:<5} Press ENTER to exit", i + 1);
        select!{
            _ = timer.recv() => {
                if !game.next() { break; }
            },
            _ = quit_thread.recv() => {
                break;
            }
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
    start(n, &initial[]);
}

