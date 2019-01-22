use std::env;

mod args;
use args::Args;

mod processor;
use processor::Processor;

type Result<T> = std::result::Result<T, String>;

fn main() {
    ::std::process::exit(match run_app(){
        Ok(()) => 0,
        Err(e) => {
            println!("{}", e);
            1
        }
    });
}

fn run_app() -> Result<()> {
    let args = Args::new(env::args().collect())?;

    // Process directory recursively
    let working_dir = args.working_dir()?;
    let mut processor = Processor::new(args.bin_size(),
                                           args.include_zeroes());
    processor.process(working_dir)?;

    // Print histogram
    processor.stats().acsii_histogram()?;


    Ok(())
}

