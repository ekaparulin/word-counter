use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Cursor;
use std::collections::HashSet;
use processor::histogram::Histogram;

extern crate zip;
mod histogram;

enum FileType {
    Text,
    Zip
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

    pub fn process(&mut self, dir: &Path) -> Option<()> {
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
                        return None;
                    }
                }
            }
        }

        Some(())
    }

    fn process_file(&mut self, path: &Path) {
        match self.file_type(path) {
            Some(FileType::Text) => {
                // Count words and store the value
                if let Some(count) = Processor::count_words_in_file(path) {
                    self.stats.add_word_count(count);
                }
            },
            Some(FileType::Zip) => {
                let mut file = File::open(path).expect("Couldn't open file");

                // Read archive into a buffer and process
                let mut buf = Vec::new();
                if let Ok(_) = file.read_to_end(&mut buf) {
                    if let Ok(mut zip_file) = zip::ZipArchive::new(Cursor::new(buf.as_slice())) {
                        self.count_words_in_zipfile(&mut zip_file);
                    }
                }
            },
            None => {
                // Do noting for usupported files
            },
        }
    }

    fn count_words_in_zipfile(&mut self, archive: &mut zip::ZipArchive<Cursor<&[u8]>>) {
        // Read Zip archive


        // Process each file in the archive
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).expect("Couldn't open archived file");

            // Process *.zip files recursively
            if (&*file.name()).ends_with(".zip") {
                // Read embedded archive into a buffer
                let mut buf = Vec::new();
                if let Ok(_) = file.read_to_end(&mut buf) {
                    if let Ok(mut zip_file) = zip::ZipArchive::new(Cursor::new(buf.as_slice())) {
                        self.count_words_in_zipfile(&mut zip_file);
                        continue;
                    }
                }

            }

            // Skip all other files but *.txt
            if !(&*file.name()).ends_with(".txt") {
                continue;
            }

            // Process *.txt files
            let mut contents = String::new();
            if let Ok(_) = file.read_to_string(&mut contents) {
                // Count words and store the value
                self.stats.add_word_count(Processor::count_words(&contents));
            }
        }
    }

    pub fn stats(&self) -> &Histogram {
        &self.stats
    }

    fn file_type(&self, path: &Path) -> Option<FileType> {
        let file_name = path.file_name().unwrap().to_os_string().into_string().unwrap();
        if file_name.ends_with("txt") {
            return Some(FileType::Text)
        } else if file_name.ends_with("zip") {
            return Some(FileType::Zip)
        }

        // Return None for unsupported file names
        None
    }

    fn count_words(contents: &String) -> usize {
        let mut words = HashSet::new();

        // Count words, iterating over white spaces
        // using hash set we'll count only unique words
        for word in contents.split_whitespace() {
            words.insert(word);
        }

        // return hash set size
        words.len()
    }

    fn count_words_in_file(path: &Path) -> Option<usize> {
        // Open and read file
        let file = File::open(path);
        if !file.is_ok() {
            return None;
        }

        let mut buf_reader = BufReader::new(file.unwrap());
        let mut contents = String::new();
        let read = buf_reader.read_to_string(&mut contents);
        if read.is_ok() {
            return Some(Processor::count_words(&contents));
        }

        None
    }

}

/* Tests */

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