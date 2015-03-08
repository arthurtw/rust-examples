#![feature(plugin, io, std_misc)]

#![plugin(regex_macros)]

extern crate regex;
// use regex::Regex;
use std::ascii::AsciiExt;
use std::collections;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;

mod config;
#[allow(unused)]
mod btree_map;

fn do_work(cfg: &config::Config) -> io::Result<()> {
    // Open input and output files
    let mut readers = Vec::with_capacity(std::cmp::max(1, cfg.input.len()));
    if cfg.input.is_empty() {
        readers.push(BufReader::new(Box::new(io::stdin()) as Box<Read>));
    } else {
        for name in &cfg.input {
            let file = try!(File::open(name));
            readers.push(BufReader::new(Box::new(file) as Box<Read>));
        }
    }
    let mut writer = match cfg.output {
        Some(ref name) => {
            let file = try!(File::create(name));
            Box::new(BufWriter::new(file)) as Box<Write>
        }
        None => { Box::new(io::stdout()) as Box<Write> }
    };

    // Parse words
    let mut map = collections::HashMap::<String, u32>::new();
    let re = regex!(r"\w+");

    // let re = Regex::new(r"\w+").unwrap();
    // let re = regex!(r"[a-zA-Z0-9_]+");
    // let re = Regex::new(r"[a-zA-Z0-9_]+").unwrap();
    for reader in &mut readers {
        for line in reader.lines() {
            for caps in re.captures_iter(&line.unwrap()) {
                if let Some(cap) = caps.at(0) {
                    let word = match cfg.ignore_case {
                        true  => cap.to_ascii_lowercase(),
                        false => cap.to_string(),
                    };
                    match map.entry(word) {
                        Occupied(mut view) => { *view.get_mut() += 1; }
                        Vacant(view) => { view.insert(1); }
                    }
                }
            }
        }
    }
    // Write counts
    let mut words: Vec<&String> = map.keys().collect();
    words.sort();
    for &word in &words {
        if let Some(count) = map.get(word) {
            try!(writeln!(writer, "{}\t{}", count, word));
        }
    }
    Ok(())
}

#[cfg(not(test))]
fn main() {
    let cfg = match config::get_config(std::env::args()) {
        Ok(c) => c,
        Err(usage) => {
            println!("{}", usage);
            return;
        }
    };
    do_work(&cfg).unwrap();
}

#[test]
fn test_do_work() {
    let bad_filename = "/..no-such-file..";
    let cfg = config::Config {
        input: vec![bad_filename.to_string()],
        output: None,
        ignore_case: false,
    };
    match do_work(&cfg) {
        // should fail for bad input file (couldn't open file ... path=/..no-such-file..; ...)
        Err(e) => assert!(format!("{}", e).as_slice().contains(bad_filename), format!("{}", e)),
        _ => panic!()
    }
}

