# word-counter

A demo of a small application, counting words in text files and archived text 
files in a given directory.

*The requirement is*:

``` 
Please write an application that will:
          
- Allow the user to provide a path to a directory
- Find all text files in that directory and its children
- If it encounters a compressed archive, open it and process any text files inside
- Output a histogram of the word counts for the files
```

*The following questions are yet top be answered:*

```
- shall the find go recursively for the given directory, or just handle the files 
  in the directory, ignoring subdirectories?
- what archive types shall be supported? ZIP, GZ, BZ, XZ? Tar balls? 
- we should handle only text files and archives, if so:
    - should all other files (binaries) be skipped?
	- how smart should the detection be, that a file is a text or archive?
		- simple by suffix?
        - by file signatures (magic numbers)?
	
- how should the histogram to be:
  - a textual listing of files in two columns, where first column is the file 
    name and the second is the word count?
 -  a rendered picture as a chart? if so:
	- in which format (PNG, SGV, JPEG?)
        - how shall the output be provides:
		- as binary stream to stdout?
                - hardcoded file name (which?), in current working directory? 
                - file path specified by additional comman line parameter?

```

*The first version will perform the requirements with following limitation:*

```
The application will:
		- not go to subdirectories
        - handle only one type of archives
        - detect text and archives by file suffix
        - print the histogram as a two column text (filename, word count)
        - no unit tests
        - simple error handling (fail fast)
```

*Building and usage:*

The application is written in Rust. To build and run, set up Rust and Cargo 
(see https://www.rust-lang.org/tools/install)and run:

    $ cargo build
    $ target/debug/word-counter "path to directory"

