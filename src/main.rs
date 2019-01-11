use std::env;

mod args;
use args::Args;

mod processor;
use processor::Processor;

fn main() {
    ::std::process::exit(match run_app(){
        true => 0,
        false => 1
    });
}

fn run_app() -> bool {
    let args = Args::new(env::args().collect());
    if !args.validate() {
        args.usage();
        return false;
    };

    let working_dir = args.working_dir();
    let mut processor = Processor::new();
    assert_eq!(processor.process(working_dir),true);

    processor.print_histogram();

    true
}

