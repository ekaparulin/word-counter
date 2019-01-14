use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use processor::histogram::Histogram;

extern crate zip;
mod histogram;

enum FileType {
    Text,
    Zip,
    Unknown
}

pub struct Processor {
    // key - word count
    // value - word count frequency
    stats: Histogram
}

impl Processor {

    pub fn new(bin_size: usize, include_zeroes: bool) -> Processor {
        Processor{
            stats: Histogram::new(bin_size, include_zeroes)
        }
    }

    // TODO: Modify error handling using Rust's Option enum
    pub fn process(&mut self, dir: &Path) -> bool {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let entry_path = entry.path();

                    if let Ok(metadata) = entry.metadata() {

                        // Recurse in directories
                        if metadata.is_dir() {
                            self.process(entry_path.as_path());
                        }

                        // Process files
                        if metadata.is_file() {
                            self.process_file(entry_path.as_path())
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

    fn process_file(&mut self, path: &Path) {
        match self.file_type(path) {
            FileType::Text => {
                // Count words and store the value
                self.stats.add_word_count(Processor::count_words_in_file(path));
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
                    //let read_result = file.read_to_string(&mut contents);
                    if let Ok(_) = file.read_to_string(&mut contents) {
                        // Count words and store the value
                        self.stats.add_word_count(Processor::count_words(&contents));
                    }
                }

            },
            FileType::Unknown => {
                // Do noting, by design
            },
        }
    }

    pub fn stats(&self) -> &Histogram {
        &self.stats
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
    fn count_words(contents: &String) -> usize {
        let mut count: usize = 0;

        // Count words, iterating over white spaces
        for _ in contents.split_whitespace() {
            count+=1;
        }

        count
    }

    fn count_words_in_file(path: &Path) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn count_words() {
        assert_eq!(Processor::count_words(&String::from("a b c")),3);
    }

    #[test]
    fn process_temp_folder() {
        let mut proc = Processor::new(1, false);
        let tmp_path = env::temp_dir().clone();
        let work_dir = Path::new(&tmp_path);
        proc.process(&work_dir);
    }
}