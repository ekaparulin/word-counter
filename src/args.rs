use std::path::Path;

extern crate getopts;
use self::getopts::Options;

pub struct Args {
    args: Vec<String>,
    working_dir: String,
    bin_size: usize,
    includes_zeroes: bool
}

impl Args {
    pub fn new(args: Vec<String>) -> Args {
        Args {
            args,
            working_dir: String::new(),
            bin_size: 1,
            includes_zeroes: false
        }
    }

    pub fn validate(&mut self) -> bool {


        let mut opts = Options::new();
        opts.optopt("b", "bin-size", "bin size (default=1)","");
        opts.optflag("z", "with-zeroes", "print out bins w/ zero frequencies");
        opts.optflag("h", "help", "print this help message");

        let matches = match opts.parse(&self.args[1..]) {
            Ok(m) => { m }
            Err(f) => { panic!(f.to_string()) }
        };

        if let Some(bin_size) = matches.opt_str("b") {
            self.bin_size = bin_size.parse().unwrap();
        }

        if matches.opt_present("z") {
            self.includes_zeroes = true;
        }

        if matches.opt_present("h") {
            self.print_usage(opts);
            return false;
        }

        self.working_dir = if !matches.free.is_empty() {
            matches.free[0].clone()
        } else {
            String::new()
        };

        if self.working_dir.is_empty() {
            self.print_usage(opts);
            return false;
        }

        if !Path::new(&self.working_dir).is_dir() {
            eprintln!("ERROR: {} is not a directory!", &self.working_dir);
            self.print_usage(opts);
            return false;
        }

        true
    }

    fn print_usage(&self, opts: Options) {
        let brief = format!("Usage: {} directory_path [options]", &self.args[0]);
        print!("{}", opts.usage(&brief));
    }

    pub fn working_dir(&self) -> &Path {
        let path = Path::new(&self.working_dir);
        assert_eq!(path.is_dir(), true);

        path
    }

    pub fn bin_size(&self) -> usize {
        self.bin_size
    }

    pub fn include_zeroes(&self) -> bool {
        self.includes_zeroes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn validate_args(args: Vec<String>) -> bool {
        let mut my_args = Args::new(args);
        my_args.validate()
    }

    #[test]
    fn has_input_directory() {
        let ret = validate_args([
                                        "program_name",
                                        env::temp_dir().to_str().unwrap(),
                                    ].iter().map(|&s| s.into()).collect());
        assert_eq!(ret, true);
    }

    #[test]
    fn include_zeroes() {
        let ret = validate_args([
                                        "program_name",
                                        "-z",
                                        env::temp_dir().to_str().unwrap()
                                    ].iter().map(|&s| s.into()).collect());
        assert_eq!(ret, true);
    }

    #[test]
    fn bin_size() {
        let ret = validate_args([
                                        "program_name",
                                        "-b100",
                                        env::temp_dir().to_str().unwrap()
                                    ].iter().map(|&s| s.into()).collect());
        assert_eq!(ret, true);
    }

    #[test]
    fn help() {
        let ret = validate_args([
                                        "program_name",
                                        "-h",
                                    ].iter().map(|&s| s.into()).collect());
        assert_eq!(ret, false);
    }
}