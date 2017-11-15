// cookbook: https://docs.rs/csv/1.0.0-beta.5/csv/cookbook/index.html
extern crate csv;
extern crate serde;
extern crate chrono;

use std::path::Path;
use std::fs::File;
use self::chrono::NaiveDate;
use std::io::{Read};
use super::rw;

#[derive(Debug,Deserialize)]
struct Record {
    pref: String,
    region: String,
    temp: f32,
    date: NaiveDate,
}

pub struct CSVExtractor<'a> {path: &'a Path}


impl<'a> CSVExtractor<'a> {
    pub fn new<P: AsRef<Path>+?Sized>(path: &P)->CSVExtractor {
        CSVExtractor {path: path.as_ref()}
    }

    fn deserialize(&self)->Vec<Record> {
        csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false) // By default, the first row is treated as a special header row,
            .from_reader(File::open(self.path).unwrap())
            .deserialize::<Record>()
            .flat_map(|s| s)
            .collect()
    }

    /// ch02.10 count lines
    pub fn count_lines(&self)->usize {
        csv::Reader::from_reader(File::open(self.path).unwrap())
            .records()
            .count()
    }

    /// ch02.11 replace a tab-character to a space
    pub fn replace_tab_to_space(&self)->String {
        csv::ReaderBuilder::new()
            .delimiter(b'\t') // The default is b','.
            .has_headers(false) // By default, the first row is treated as a special header row,
            .from_reader(File::open(self.path).unwrap())
            .into_records()
            .map(|s|
                s.unwrap()
                    .iter()
                    .collect::<Vec<&str>>()
                    .join(" ") // space
            )
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// helper for ch02.12
    /// n: col index beginning with 0.
    pub fn save_first_second_row<P: AsRef<Path>>(&self, file1: P, file2: P) {
        let first_row = self.deserialize()
            .into_iter()
            .map(|s| s.pref)
            .collect::<Vec<_>>()
            .join("\n");

        rw::write(first_row, file1.as_ref());

        let second_row = self.deserialize()
            .into_iter()
            .map(|s| s.region)
            .collect::<Vec<_>>()
            .join("\n");

        rw::write(second_row, file2.as_ref());
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use ch02::command::Commander;
    #[test]
    fn test_new() {
        let csvor = CSVExtractor::new("./data/ch02/hightemp.txt");
    }

    #[test]
    fn test_count_lines() {
        let path = "./data/ch02/hightemp.txt";
        let csvor = CSVExtractor::new(path);

        assert_eq!(
            csvor.count_lines(),
            Commander::new(path).count_lines().unwrap()
        );
    }

    #[test]
    fn test_replace_tab_to_space() {
        let path = "./data/ch02/hightemp.txt";
        let csvor = CSVExtractor::new(path);

        let commander = Commander::new(path);

        assert_eq!(
            csvor.replace_tab_to_space(),
            commander.replace_tab_to_space().trim()
        )
    }

    #[test]
    fn test_save_first_second_row() {
        let load_path = Path::new("./data/ch02/hightemp.txt");
        let parent = load_path.parent().unwrap();

        let file1 = parent.join("col1.txt");
        let file2 = parent.join("col2.txt");

        // assume that file doesn't exist
        let _ = vec![&file1, &file2]
            .into_iter()
            .map(|fpath| ::std::fs::remove_file(fpath))
            .collect::<Vec<_>>();

        let csvor = CSVExtractor::new(load_path);

        csvor.save_first_second_row(&file1, &file2);

        assert!(file1.exists());
        assert!(file2.exists());

        let commander = Commander::new(load_path);

        assert_eq!(commander.extract_row(1),
                   rw::read(&file2)
                       .unwrap()
        );
    }
}
