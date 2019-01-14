# word-counter

A demo of a small application, counting words in text files and archived text 
files in a given directory.

## requirement

``` 
    Please write an application that will:

        - Allow the user to provide a path to a directory
        - Find all text files (.txt) in that directory and its children (recursively)
        - If it encounters a compressed archive (.zip), open it and process any text
          files inside
        - Output a histogram of the word counts for the files
```

## build

The application is written in Rust. To build and run, set up Rust and Cargo
(see https://www.rust-lang.org/tools/install), enter the directory containing
the Cargo.toml file and src directory and type:

    $ cargo build

## run

To run the binary:

    $ target/debug/word-counter "path to directory"

This will output a histogram, with binsize of 1, excluding frequencies equal zero.

### options

To view the command usage, you can pass a -h flag to it:



Note: The options must be specified _after_ the directory path!

### bin size

To specify the bin size, you can add the -b parameter, e.g. -b10 for bin size
of 10 words:

    $ target/debug/word-counter "path to directory" -b10

### include zero frequiencies

To draw a full histogram, including all bins, w/o any frequencies (empty values),
add the -z flag:

    $ target/debug/word-counter "path to directory" -z

You can combine the flags:

    $ target/debug/word-counter "path to directory" -z -b10

This will draw full histogram with 10 word bin size.