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
}