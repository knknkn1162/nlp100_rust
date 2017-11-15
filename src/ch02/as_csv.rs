// cookbook: https://docs.rs/csv/1.0.0-beta.5/csv/cookbook/index.html
extern crate csv;
extern crate serde;
extern crate chrono;

use std::path::Path;
use std::fs::File;
use self::chrono::NaiveDate;
use std::io::{Read};
use super::rw;
use std::fmt::Display;

#[derive(Debug,Deserialize, Serialize)]
struct Record {
    pref: String,
    region: String,
    temp: f32,
    date: NaiveDate,
}

pub struct CSVExtractor<'a> {path: &'a Path}


impl<'a> CSVExtractor<'a> {
    pub fn new<P: AsRef<Path> + ? Sized>(path: &P) -> CSVExtractor {
        CSVExtractor { path: path.as_ref() }
    }

    fn deserialize(&self) -> Vec<Record> {
        csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .has_headers(false) // By default, the first row is treated as a special header row,
            .from_reader(File::open(self.path).unwrap())
            .deserialize::<Record>()
            .flat_map(|s| s)
            .collect()
    }

    /// ch02.10 count lines
    pub fn count_lines(&self) -> usize {
        csv::Reader::from_reader(File::open(self.path).unwrap())
            .records()
            .count()
    }

    /// ch02.11 replace a tab-character to a space
    pub fn replace_tab_to_space(&self) -> String {
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

    /// helper for ch02.12; extract first & second row and return (String, String)
    fn extract_first_second_row(&self) -> (String, String) {
        let first_row = self.deserialize()
            .into_iter()
            .map(|s| s.pref)
            .collect::<Vec<_>>()
            .join("\n");


        let second_row = self.deserialize()
            .into_iter()
            .map(|s| s.region)
            .collect::<Vec<_>>()
            .join("\n");

        (first_row, second_row)
    }

    /// ch02.12; save first and second row in each file
    pub fn save_first_second_row<P: AsRef<Path>>(&self, pref_file: P, region_file: P) {
        let (prefs, regions) = self.extract_first_second_row();

        rw::write(prefs, pref_file);
        rw::write(regions, region_file);
    }

    /// ch03.13; merge col1.txt and col2.txt and save on file
    pub fn save_merge<P1: AsRef<Path>, P2: AsRef<Path>>(file1: P1, file2: P1, save_path: P2) {
        let (row1, row2) = (rw::read_lines(file1).unwrap(), rw::read_lines(file2).unwrap());
        rw::write(
            merge(&row1, &row2, '\t'),
            save_path
        );
    }

    // ch02.14~16 same as ch02.rs

    /// ch02.17 collect unique items in first row.
    pub fn uniq_first_row(&self)->String {
        let mut prefs = self.deserialize()
            .into_iter()
            .map(|s| s.pref)
            .collect::<Vec<_>>();
        prefs.sort_unstable();
        prefs.dedup();
        prefs.join("\n")
    }

    /// ch02.18 sort by third columns in descending
    pub fn sort_temp_in_descending(&self)->String {
        let mut records = self.deserialize();
        records.sort_by(|s1, s2|
            s2.temp.partial_cmp(&(s1.temp)).unwrap()
        );

        self::serialize(&records, '\t')
    }
}

/// helper for ch03.13; merge col1.txt and col2.txt
fn merge<S1: ToString, S2: ToString>(row1: &[S1], row2: &[S2], delimiter: char)->String {
    row1.into_iter()
        .zip(row2.into_iter())
        .map(|(s1, s2)|
            format!("{}{}{}", s1.to_string(), delimiter, s2.to_string()))
        .collect::<Vec<String>>()
        .join("\n")
}

/// serialize
fn serialize(records: &[Record], delimiter: char)->String {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(delimiter as u8)
        .has_headers(false)
        .from_writer(vec![]);

    records.into_iter()
        .for_each(|record| {
            let _ = wtr.serialize(record);
        });
    String::from_utf8(wtr.into_inner().unwrap())
        .unwrap()

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
    fn test_extract_first_second_row() {
        let path = "./data/ch02/hightemp.txt";
        let csvor = CSVExtractor::new(path);

        let (prefs, regions) = csvor.extract_first_second_row();

        let commander = Commander::new(path);

        assert_eq!(
            commander.extract_row(0),
            prefs
        );

        assert_eq!(
            commander.extract_row(1),
            regions
        );
    }

    #[test]
    fn test_save_first_second_row() {
        let load_path = Path::new("./data/ch02/hightemp.txt");
        let parent = load_path.parent().unwrap();

        let file1 = parent.join("col1.txt");
        let file2 = parent.join("col2.txt");

        let csvor = CSVExtractor::new(load_path);

        // assume that file doesn't exist
        vec![&file1, &file2]
            .into_iter()
            .for_each(|fpath| {let _ = ::std::fs::remove_file(fpath);});

        csvor.save_first_second_row(&file1, &file2);

        // assert files exist
        assert!(file1.exists());
        assert!(file2.exists());
    }

    #[test]
    fn test_merge() {
        let row1 = vec!["aa", "bb", "cc"];
        let row2 = [11, 12, 13];
        let res = merge(&row1, &row2[..], '\t');

        assert_eq!(
            res,
            "aa\t11\nbb\t12\ncc\t13"
        )
    }

    #[test]
    fn test_save_merge() {
        let load_path = Path::new("./data/ch02/hightemp.txt");
        let parent = load_path.parent().unwrap();

        let file1 = parent.join("col1.txt");
        let file2 = parent.join("col2.txt");
        let save_path = parent.join("col_12.txt");


        // assume that save_path doesn't exist.
        let _ = ::std::fs::remove_file(&save_path).unwrap();

        CSVExtractor::save_merge(file1, file2, &save_path);

        assert!(save_path.exists());
    }

    #[test]
    fn test_uniq_first_row() {
        let load_path = Path::new("./data/ch02/hightemp.txt");
        let csvor = CSVExtractor::new(&load_path);

        let commander = Commander::new(&load_path);

        assert_eq!(
            csvor.uniq_first_row(),
            commander.uniq_first_row()
        );
    }

    #[test]
    fn test_sort_temp_in_descending() {
        let load_path = Path::new("./data/ch02/hightemp.txt");
        let csvor = CSVExtractor::new(&load_path);

        let res = csvor.sort_temp_in_descending();

        assert_eq!(
            res.lines().take(3).collect::<Vec<&str>>(),
            vec!["高知県\t江川崎\t41\t2013-08-12", "埼玉県\t熊谷\t40.9\t2007-08-16", "岐阜県\t多治見\t40.9\t2007-08-16"]
        )
    }

}
