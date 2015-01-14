// Uncomment the following line to disable unstable warnings:
// #![allow(unstable)]

#![feature(plugin)]
#[plugin] #[no_link]
extern crate regex_macros;
extern crate regex;
use std::ascii::AsciiExt;
use std::collections;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::io;
use std::io::{BufferedReader, File, IoResult};

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
        Some(ref x) => { Box::new(try!(File::create(&Path::new(x.as_slice())))) as Box<Writer> }
        None => { Box::new(io::stdout()) as Box<Writer> }
    };

    // Parse words
    let mut map = collections::HashMap::<String, u32>::new();
    // let mut map = btree_map::BTreeMap::<String, u32>::new();
    let re = regex!(r"\w+");
    // let re = Regex::new(r"\w+").unwrap();
    for reader in readers.iter_mut() {
        for line in reader.lines() {
            for caps in re.captures_iter(line.unwrap().as_slice()) {
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
    for word in words.iter() {
        if let Some(count) = map.get(*word) {
            let line = format!("{}\t{}\n", count, word);
            try!(writer.write(line.as_bytes()));
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

