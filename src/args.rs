use std::path::Path;


extern crate getopts;
use self::getopts::Options;

use std::result;
type Result<T> = result::Result<T, String>;

pub struct Args {
    args: Vec<String>,
    opts: Options,
    working_dir: String,
    bin_size: usize,
    includes_zeroes: bool
}

impl Args {
    pub fn new(args: Vec<String>) -> Result<Args> {
        let mut args = Args {
            args,
            opts: Options::new(),
            working_dir: String::new(),
            bin_size: 1,
            includes_zeroes: false
        };
        args.validate()?;

        Ok(args)

    }

    fn validate(&mut self) -> Result<()> {

        self.opts.optopt("b", "bin-size", "bin size (default=1)","");
        self.opts.optflag("z", "with-zeroes", "print out bins w/ zero frequencies");
        self.opts.optflag("h", "help", "print this help message");

        let matches = self.opts.parse(&self.args[1..]).unwrap();

        if let Some(bin_size) = matches.opt_str("b") {
            self.bin_size = bin_size.parse().unwrap();
        }

        if matches.opt_present("z") {
            self.includes_zeroes = true;
        }

        if matches.opt_present("h") {
            return Err(self.usage_message());
        }

        self.working_dir = if !matches.free.is_empty() {
            matches.free[0].clone()
        } else {
            String::new()
        };

        // Check if working dir is valid
        match self.working_dir() {
            Err(e) => return Err(e),
            Ok(_) => Ok(())
        }

    }

    fn usage_message(&self) -> String {
        format!("{}", self.opts.usage(
            &format!("Usage: {} directory_path [options]", &self.args[0])))
    }

    pub fn working_dir(&self) -> Result<&Path> {

        if self.working_dir.is_empty() {
            return Err(format!("ERROR: Directory '{}' is empty!\n{}",
                               self.working_dir, self.usage_message()));
        }

        let path = Path::new(&self.working_dir);
        if !path.is_dir() {
            return Err(format!("ERROR: '{}' is not a directory!\n{}",
                               self.working_dir, self.usage_message()));
        }

        Ok(path)
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

    #[test]
    fn has_input_directory() {
        let ret = Args::new([
                                        "program_name",
                                        env::temp_dir().to_str().unwrap(),
                                    ].iter().map(|&s| s.into()).collect());
        assert!(ret.is_ok());
    }

    #[test]
    fn include_zeroes() {
        let ret = Args::new([
                                        "program_name",
                                        "-z",
                                        env::temp_dir().to_str().unwrap()
                                    ].iter().map(|&s| s.into()).collect());
        assert!(ret.is_ok());
    }

    #[test]
    fn bin_size() {
        let ret = Args::new([
                                        "program_name",
                                        "-b100",
                                        env::temp_dir().to_str().unwrap()
                                    ].iter().map(|&s| s.into()).collect());
        assert!(ret.is_ok());
    }

    #[test]
    fn help() {
        let ret = Args::new([
                                        "program_name",
                                        "-h",
                                    ].iter().map(|&s| s.into()).collect());
        assert!(ret.is_err());
    }
}