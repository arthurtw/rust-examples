// Uncomment the following line to disable unstable warnings:
// #![allow(unstable)]

#![feature(plugin)]
#[plugin] #[no_link]
extern crate regex_macros;
extern crate regex;
// use regex::Regex;
use std::ascii::AsciiExt;
use std::collections;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::io;
use std::io::{BufferedReader, BufferedWriter, File, IoResult};

pub mod config;
pub mod btree_map;

fn do_work(cfg: &config::Config) -> IoResult<()> {
    // Open input and output files
    let mut readers = vec![];
    if cfg.input.is_empty() {
        readers.push(BufferedReader::new(Box::new(io::stdin()) as Box<Reader>));
    } else {
        for name in cfg.input.iter() {
            let file = try!(File::open(&Path::new(name.as_slice())));
            readers.push(BufferedReader::new(Box::new(file) as Box<Reader>));
        }
    }
    let mut writer = match cfg.output {
        Some(ref x) => { Box::new(BufferedWriter::new(try!(File::create(&Path::new(x.as_slice()))))) as Box<Writer> }
        None => { Box::new(io::stdout()) as Box<Writer> }
    };

    // Parse words
    let mut map = collections::HashMap::<String, u32>::new();
    // let mut map = btree_map::BTreeMap::<String, u32>::new();
    let re = regex!(r"\w+");
    // let re = Regex::new(r"\w+").unwrap();
    // let re = regex!(r"[a-zA-Z0-9_]+");
    // let re = Regex::new(r"[a-zA-Z0-9_]+").unwrap();
    for reader in readers.iter_mut() {
        for line in reader.lines() {
            let line = line.unwrap();
            for (start, end) in re.find_iter(&line[]) {
                let word = &line[start..end];
                let word = if cfg.ignore_case { word.to_ascii_lowercase() }
                            else { String::from_str(word) };
                match map.entry(word) {
                    Occupied(mut view) => { *view.get_mut() += 1; }
                    Vacant(view) => { view.insert(1); }
                }
            }
        }
    }

    // Write counts
    let mut words: Vec<&String> = map.keys().collect();
    words.sort();
    for word in words.iter() {
        if let Some(count) = map.get(*word) {
            try!(writeln!(writer, "{}\t{}", count, word));
        }
    }
    Ok(())
}

#[cfg(not(test))]
fn main() {
    let cfg = match config::get_config(std::os::args()) {
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

