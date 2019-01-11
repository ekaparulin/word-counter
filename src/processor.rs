use std::path::Path;
use std::vec::Vec;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

extern crate zip;

enum FileType {
    Text,
    Zip,
    Unknown
}

pub struct Processor {
    stats: Vec<(String, u64)>
}

impl Processor {

    pub fn new() -> Processor {
        Processor{
            stats: Vec::new()
        }
    }

    // TODO: Modify error handling using Rust's Option enum
    pub fn process(&mut self, dir: &Path) -> bool {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_file() {
                            self.process_file(entry.path().as_path())
                        }
                    } else {
                        eprintln!("Couldn't get file metadata for {:?}", entry.path());
                        return false;
                    }
                }
            }
        }

        true
    }

    pub fn print_histogram(&self) {
        for (file, size) in &self.stats {
            println!("{}: {}", file, size);
        }
    }

    fn process_file(&mut self, path: &Path) {
        match self.file_type(path) {
            FileType::Text => {
                // Add stats for simple text file
                self.add_stats(&path.file_name().unwrap().to_os_string().into_string().unwrap(),
                               Processor::count_words_in_file(path));
            },
            FileType::Zip => {

                // Read Zip archive
                let file = fs::File::open(path).unwrap();
                let mut archive = zip::ZipArchive::new(file).unwrap();

                // Process each file in the archive
                for i in 0..archive.len() {
                    let mut file = archive.by_index(i).unwrap();

                    // Skip all non *.txt files
                    if !(&*file.name()).ends_with(".txt") {
                        continue;
                    }

                    let mut contents = String::new();
                    let read_result = file.read_to_string(&mut contents);
                    assert_eq!(read_result.is_ok(), true);

                    // Add stats as ZIP_file[file.txt]
                    self.add_stats(&format!("{}[{}]",
                                           &path.file_name().unwrap().to_os_string().into_string().unwrap(),
                                           file.sanitized_name().into_os_string().into_string().unwrap()),
                                   Processor::count_words(&contents));
                }

            },
            FileType::Unknown => {
                // Do noting, by design
            },
        }
    }

    fn add_stats(&mut self, name: &String, count: u64) {
        self.stats.push((name.to_string(), count));
    }

    /* "static" methods */
    // TODO: Make a better use of Rust enums here
    fn file_type(&self, path: &Path) -> FileType {
        let file_name = path.file_name().unwrap().to_os_string().into_string().unwrap();
        if file_name.ends_with("txt") {
            return FileType::Text
        } else if file_name.ends_with("zip") {
            return FileType::Zip
        }

        FileType::Unknown
    }

    // TODO: Handle errors with Rust Option enum rather than with enums
    fn count_words(contents: &String) -> u64 {
        let mut count: u64 = 0;

        // Count words, iterating over white spaces
        for _ in contents.split_whitespace() {
            count+=1;
        }

        count
    }

    fn count_words_in_file(path: &Path) -> u64 {
        // Open and read file
        let file = File::open(path);
        assert_eq!(file.is_ok(), true);
        let mut buf_reader = BufReader::new(file.unwrap());
        let mut contents = String::new();
        let read = buf_reader.read_to_string(&mut contents);
        assert_eq!(read.is_ok(), true);

        return Processor::count_words(&contents);
    }

}