extern crate getopts;
use self::getopts::{optopt, optflag, OptGroup};

pub type ConfigResult = Result<Config, String>;

#[derive(Show)]
pub struct Config {
    pub input: Vec<String>,
    pub output: Option<String>,
    pub ignore_case: bool,
}

fn get_usage(program: &str, opts: &[OptGroup]) -> String {
    let prgm_path = Path::new(program);
    let prgm_name = prgm_path.filename_str().unwrap();
    let brief = format!("Usage: {} [OPTIONS] [FILES]", prgm_name);
    // let brief = format!("Usage: {} [OPTIONS] INPUT\n\t(for stdin INPUT, use \"-\")", prgm_name);
    getopts::usage(brief.as_slice(), opts)
}

pub fn get_config(args: Vec<String>) -> ConfigResult {
    let program = args[0].as_slice();

    let opts = &[
        optopt("o", "", "set output file name", "NAME"),
        optflag("i", "ignore-case", "ignore case"),
        optflag("h", "help", "print this help menu"),
    ];
    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(_) => { return Err(get_usage(program, opts)) }
    };
    if matches.opt_present("h") {
        return Err(get_usage(program, opts));
    }

    Ok(Config {
        input: matches.free.clone(),
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

