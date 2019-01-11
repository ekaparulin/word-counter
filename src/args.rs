use std::path::Path;

pub struct Args {
    args: Vec<String>
}

impl Args {
    pub fn new(args: Vec<String>) -> Args {
        Args {
            args,
        }
    }

    // TODO: Modify error handling using Rust's Option enum and custom errors
    pub fn validate(&self) -> bool {
        if self.args.len() > 2 {
            print!("Error: Missing directory path!");
            return false;
        }
        true
    }

    pub fn usage(&self) {
        println!("Usage: {} directory_path", &self.args[0]);
    }

    // TODO: Replace asserts using Rust's Option enum and custom errors
    pub fn working_dir(&self) -> &Path {
        assert_eq!(self.args.len() > 1, true);
        let path = Path::new(&self.args[1]);
        assert_eq!(path.is_dir(), true);

        path
    }
}
