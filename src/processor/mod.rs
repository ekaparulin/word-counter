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

use std::result;
type Result<T> = result::Result<T, String>;

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

    pub fn process(&mut self, dir: &Path) -> Result<()> {
         match fs::read_dir(dir) {
             Ok(res) => {
                 for read_result in res {
                     if read_result.is_err() {
                         return Err(format!("ERROR: Couldn't read directory {}", dir.to_str().unwrap()));
                     }

                     let entry = &read_result.unwrap();
                     let entry_path = entry.path();
                     let metadata = entry.metadata().unwrap();

                     // Recurse in directories
                     if metadata.is_dir() {
                         self.process(entry_path.as_path())?;
                     } else if metadata.is_file() { // Process files
                         self.process_file(entry_path.as_path())?;
                     }
                 }
             },
             Err(e) => return Err(format!("ERROR: '{}': {}", dir.to_str().unwrap(), e.to_string()))
         }

        Ok(())
    }

    fn process_file(&mut self, path: &Path) -> Result<()> {
        match self.file_type(path) {
            Some(FileType::Text) => {
                // Count words and store the value
                self.count_words_in_file(path)
            },
            Some(FileType::Zip) => {
                match File::open(path) {
                    Ok(mut file) => {
                        // Read archive into a buffer and process
                        let mut buf = Vec::new();
                        if let Ok(_) = file.read_to_end(&mut buf) {
                            if let Ok(mut zip_file) = zip::ZipArchive::new(Cursor::new(buf.as_slice())) {
                                return self.count_words_in_zipfile(&mut zip_file);
                            }
                        }
                        return Ok(())
                    },
                    Err(e) => return Err(format!("ERROR: '{}': {}",
                                                 path.file_name().unwrap().to_os_string().into_string().unwrap(),
                                                 e.to_string()))
                }
            },
            None => {
                Ok(())
            },
        }
    }

    fn count_words_in_zipfile(&mut self, archive: &mut zip::ZipArchive<Cursor<&[u8]>>) -> Result<()>{
        // Process each file in the archive
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).expect("Couldn't open archived file");

            // Process *.zip files recursively
            if (&*file.name()).ends_with(".zip") {
                // Read embedded archive into a buffer
                let mut buf = Vec::new();
                if let Ok(_) = file.read_to_end(&mut buf) {
                    if let Ok(mut zip_file) = zip::ZipArchive::new(Cursor::new(buf.as_slice())) {
                        self.count_words_in_zipfile(&mut zip_file)?;
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

        Ok(())
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

    fn count_words_in_file(&mut self, path: &Path) -> Result<()> {
        // Open and read file
        let file = File::open(path);
        if file.is_err() {
            return Err(format!("ERROR: '{}': {}",
                               path.file_name().unwrap().to_os_string().into_string().unwrap(),
                               file.err().unwrap()))
        }

        let mut buf_reader = BufReader::new(file.unwrap());
        let mut contents = String::new();

        match buf_reader.read_to_string(&mut contents) {
            Ok(_) => {
                self.stats.add_word_count(Processor::count_words(&contents));
                return Ok(())
            },
            Err(e) => return Err(format!("ERROR: '{}': {}",
                                  path.file_name().unwrap().to_os_string().into_string().unwrap(),
                                  e.to_string()))
        }
  }

}

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn count_words() {
        assert_eq!(Processor::count_words(&String::from("a b c a")),3);
    }

    #[test]
    fn process_temp_folder() {
        let mut proc = Processor::new(1, false);
        let tmp_path = env::temp_dir().clone();
        let work_dir = Path::new(&tmp_path);
        assert!(proc.process(&work_dir).is_ok());
    }
}