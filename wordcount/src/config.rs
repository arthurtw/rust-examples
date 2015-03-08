extern crate getopts;
use self::getopts::Options;

pub type ConfigResult = Result<Config, String>;

#[derive(Debug)]
pub struct Config {
    pub input: Vec<String>,
    pub output: Option<String>,
    pub ignore_case: bool,
}

fn get_usage(program: &str, opts: Options) -> String {
    let brief = format!("Usage: {} [OPTIONS] [FILES]", program);
    // let brief = format!("Usage: {} [OPTIONS] INPUT\n\t(for stdin INPUT, use \"-\")", prgm_name);
    opts.usage(&brief)
}

pub fn get_config<I: Iterator<Item=String>>(mut args: I) -> ConfigResult {
    let program = args.next().unwrap();

    let mut opts = Options::new();
    opts.optopt("o", "output", "set output file name", "NAME")
        .optflag("i", "ignore-case", "ignore case")
        .optflag("h", "help", "print this help menu");

    let mut matches = match opts.parse(args) {
        Ok(m) => { m }
        Err(_) => { return Err(get_usage(&program, opts)) }
    };
    if matches.opt_present("h") {
        return Err(get_usage(&program, opts));
    }

    let free = ::std::mem::replace(&mut matches.free, Vec::new());

    Ok(Config {
        input: free,
        output: matches.opt_str("o"),
        ignore_case: matches.opt_present("i"),
    })
}

#[cfg(test)]
mod test {
    use super::get_config;

    fn to_args(mut v: Vec<&str>) -> Vec<String> {
        v.insert(0, "wordcount"); // default args[0] to "wordcount"
        v.iter().map(|s| s.to_string()).collect()
    }

    fn check_usage(usage: String) { assert!(usage.as_slice().contains("Usage:"), usage) }

    #[test]
    fn test_get_config_usage() {
        match get_config(to_args(vec!["-h"])) {
            Err(e) => check_usage(e),
            _ => panic!(),
        }
        match get_config(to_args(vec!["--bad-option", "input"])) {
            Err(e) => check_usage(e),
            _ => panic!(),
        }
    }

    #[test]
    fn test_get_config_normal() {
        match get_config(to_args(vec!["-i", "-o", "outfile", "infile"])) {
            Ok(c) => {
                assert!(c.ignore_case);
                assert_eq!(c.input, vec!["infile".to_string()]);
                assert_eq!(c.output, Some("outfile".to_string()));
            }
            _ => panic!(),
        }
    }
}

