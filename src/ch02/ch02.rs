use std::io::{BufReader, Read, LineWriter, Write};
use std::fs::File;
use std::io;
use std::path::Path;

struct FileExtractor<'a> {path: &'a str}

impl<'a> FileExtractor<'a> {
    pub fn new(path: &'a str)-> FileExtractor<'a> {
        FileExtractor {path: path}
    }

    fn read(&self)->Result<String,io::Error>  {
        let f = File::open(self.path)?; // read only.
        let mut reader = BufReader::new(f);

        let mut line = String::new();
        let len = reader.read_to_string(&mut line)?;

        debug!("{} characters", len);
        Ok(line)
    }

    /// ch01.10 count lines
    pub fn count_lines(&self)->usize {
        let s = self.read().unwrap();
        s.lines().collect::<Vec<_>>().len()
    }

    /// ch01.11 replace a tab-character to a space
    pub fn replace_tab_to_space(&self)->String {
        let s = self.read().unwrap();
        s.replace("\t", " ")
    }

    /// helper for ch02.12
    fn extract_row<T: AsRef<Path>>(&self, n: usize, file: &T) {
        let s = self.read().unwrap();

        let file = File::create(file).unwrap();
        let data = s.lines()
            .map(|line| {
                line.split('\t')
                    .nth(n)
                    .unwrap()
            })
            .collect::<Vec<_>>().join("\n");
        let _ = LineWriter::new(file).write(data.as_bytes());
    }

    /// ch02.12; save first and second row in each file
    pub fn extract_first_second_row<T: AsRef<Path>>(&self, file1: &T, file2: &T) {
        let s = self.read().unwrap();

        vec![file1, file2]
            .into_iter()
            .enumerate()
            .map(|(idx, file)|
                self.extract_row(idx, file)
            ).collect::<Vec<_>>();
    }


}

#[cfg(test)]
mod test {
    use ch02::command::Commander;
    use super::*;
    #[test]
    fn test_read() {
        let fext = FileExtractor {path: "./data/ch02/hightemp.txt"};

        let res = fext.read().unwrap();

        assert_eq!(
            res.lines().take(1).collect::<String>(),
            "高知県\t江川崎\t41\t2013-08-12"
        );
    }

    #[test]
    fn test_ch02_10_count_lines() {
        let path = "./data/ch02/hightemp.txt";
        let fxt = FileExtractor {path: path};

        let commander = Commander::new(path);
        assert_eq!(fxt.count_lines(), commander.count_lines().unwrap())
    }

    #[test]
    fn test_ch02_11_replace_tab_to_space() {
        let path = "./data/ch02/hightemp.txt";
        let fxt = FileExtractor {path: path};

        let commander = Commander::new(path);

        assert_eq!(
            fxt.replace_tab_to_space(),
            commander.replace_tab_to_space()
        )
    }

    #[test]
    fn test_ch02_12_helper_extract_row() {
        let load_path = Path::new("./data/ch02/hightemp.txt");
        let fxt = FileExtractor {path: load_path.to_str().unwrap()};

        let save_path = load_path.parent().unwrap().join("col1.txt");
        // assume that file doesn't exist
        let _ = ::std::fs::remove_file(&save_path);

        fxt.extract_row(0, &save_path);

        assert!(save_path.exists());

        let mut line = String::new();
        let mut reader = BufReader::new(File::open(save_path).unwrap());
        let _ = reader.read_to_string(&mut line);

        let commander = Commander::new(load_path);

        assert_eq!(commander.extract_row(0), line);
    }

    #[test]
    fn test_ch02_12_extract_first_second_row() {
        let load_path = Path::new("./data/ch02/hightemp.txt");
        let parent = load_path.parent().unwrap();

        // assume that file doesn't exist
        let file1 = parent.join("col1.txt");
        let file2 = parent.join("col2.txt");

        let _ = ::std::fs::remove_file(&file1);
        let _ = ::std::fs::remove_file(&file2);

        let _ = vec![&file1, &file2]
            .into_iter()
            .map(|fpath| ::std::fs::remove_file(fpath)).collect::<Vec<_>>();

        let fxt = FileExtractor {path: load_path.to_str().unwrap()};
        fxt.extract_first_second_row(&file1, &file2);

        assert!(file1.exists());
        assert!(file2.exists());

        let mut line = String::new();
        let mut reader = BufReader::new(File::open(file2).unwrap());
        let _ = reader.read_to_string(&mut line);

        let commander = Commander::new(load_path);

        assert_eq!(commander.extract_row(1), line);
    }
}