// cookbook: https://docs.rs/csv/1.0.0-beta.5/csv/cookbook/index.html
extern crate csv;
extern crate serde;
extern crate chrono;

use std::path::Path;
use std::fs::File;
use self::chrono::NaiveDate;
use std::io::{Read};

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

    /// ch02.10 count lines
    pub fn count_lines(&self)->usize {
        csv::Reader::from_reader(File::open(self.path).unwrap())
            .records()
            .count()
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
}
