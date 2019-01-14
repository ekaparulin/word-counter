use std::env;

mod args;
use args::Args;

mod processor;
use processor::Processor;

fn main() {
    ::std::process::exit(match run_app(){
        Some(()) => 0,
        None => 1
    });
}

fn run_app() -> Option<()> {
    let mut args = Args::new(env::args().collect());

    // Validate args and options
    if None == args.validate() {
        return None;
    };

    // Process directory
    if let Some(working_dir) = args.working_dir() {
        let mut processor = Processor::new(args.bin_size(),
                                           args.include_zeroes());

        if Some(()) == processor.process(working_dir) {
            processor.stats().acsii_histogram();
            return Some(())
        }
    }

    None
}

