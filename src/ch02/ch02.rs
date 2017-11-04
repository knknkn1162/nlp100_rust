use std::io::{BufReader, Read, BufWriter, Write};
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
    fn extract_row(&self, n: usize)->String {
        let s = self.read().unwrap();
        s.lines()
            .map(|line| {
                line.split('\t')
                    .nth(n)
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// ch02.12; save first and second row in each file
    pub fn save_first_second_row<T: AsRef<Path>>(&self, file1: &T, file2: &T) {
        vec![file1, file2]
            .into_iter()
            .enumerate()
            .map(|(idx, file)| {
                let f = File::create(file).unwrap();
                let mut buffer = BufWriter::new(f);
                buffer.write_all(self.extract_row(idx).as_bytes())
            })
            .collect::<Vec<_>>();
    }

    /// helper for ch03.13; merge col1.txt and col2.txt
    fn merge(row1: &str, row2: &str)-> String {
        row1.lines()
            .zip(row2.lines())
            .map(|(item1, item2)| format!("{}{}{}", item1, '\t', item2))
            .collect::<Vec<_>>().join("\n")
    }

    /// ch03.13; save result of merge method.
    pub fn save_merge<T: AsRef<Path>>(file1: &T, file2: &T, save_file: &T) {
        let mut reader = BufReader::new(File::open(file1).unwrap());
        let mut line1 = String::new();
        let _ = reader.read_to_string(&mut line1).unwrap();

        let mut reader = BufReader::new(File::open(file2).unwrap());
        let mut line2 = String::new();
        let _ = reader.read_to_string(&mut line2).unwrap();

        let res = FileExtractor::merge(&line1, &line2);

        let mut writer = BufWriter::new(File::create(save_file).unwrap());
        let _ = writer.write(res.as_bytes()).unwrap();
    }

    /// ch02.14 take first ${num} lines
    pub fn head(&self, n: usize)->String {
        let s = self.read().unwrap();
        s.lines().take(n).collect::<Vec<_>>().join("\n")
    }

    /// ch02.15 tail last ${num} lines
    pub fn tail(&self, n: usize)->String {
        let s = self.read().unwrap();

        let mut r = s.lines().rev().take(n).collect::<Vec<_>>();
        r.reverse();
        r.join("\n")
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

        let commander = Commander::new(load_path);

        assert_eq!(
            commander.extract_row(0),
            fxt.extract_row(0)
        );
    }

    #[test]
    fn test_ch02_12_extract_first_second_row() {
        let load_path = Path::new("./data/ch02/hightemp.txt");
        let parent = load_path.parent().unwrap();

        let file1 = parent.join("col1.txt");
        let file2 = parent.join("col2.txt");

        // assume that file doesn't exist
        let _ = vec![&file1, &file2]
            .into_iter()
            .map(|fpath| ::std::fs::remove_file(fpath)).collect::<Vec<_>>();

        let fxt = FileExtractor {path: load_path.to_str().unwrap()};
        fxt.save_first_second_row(&file1, &file2);

        assert!(file1.exists());
        assert!(file2.exists());

        // confirm that content of file2 is equivalent to cut command.
        let mut line = String::new();
        let mut reader = BufReader::new(File::open(file2).unwrap());
        let _ = reader.read_to_string(&mut line);

        let commander = Commander::new(load_path);

        assert_eq!(commander.extract_row(1), line);
    }

    #[test]
    fn test_merge() {
        let load_path = Path::new("./data/ch02/hightemp.txt");
        let fxt = FileExtractor {path: load_path.to_str().unwrap()};

        let lines1 = fxt.extract_row(0);
        let lines2 = fxt.extract_row(1);

        let commander = Commander::new(load_path);
        let parent = load_path.parent().unwrap();

        let file1 = parent.join("col1.txt");
        let file2 = parent.join("col2.txt");
        assert_eq!(
            FileExtractor::merge(&lines1, &lines2),
            Commander::merge(&file1, &file2)
        )
    }

    #[test]
    fn test_save_merge() {
        let load_path = Path::new("./data/ch02/hightemp.txt");
        let parent = load_path.parent().unwrap();

        let file1 = parent.join("col1.txt");
        let file2 = parent.join("col2.txt");
        let save_file = parent.join("col12.txt");

        let fxt = FileExtractor {path: load_path.to_str().unwrap()};
        fxt.save_first_second_row(&file1, &file2);

        // assume that save_file doesn't exist.
        let _ = ::std::fs::remove_file(&save_file);
        FileExtractor::save_merge(&file1, &file2, &save_file);

        assert!(save_file.exists());
    }

    #[test]
    fn test_head() {
        let load_path = Path::new("./data/ch02/hightemp.txt");

        let fxt = FileExtractor {path: load_path.to_str().unwrap()};
        let n = 5;

        let commander = Commander::new(load_path);
        assert_eq!(
            fxt.head(n),
            commander.head(n)
        )
    }

    #[test]
    fn test_tail() {
        let load_path = Path::new("./data/ch02/hightemp.txt");

        let fxt = FileExtractor {path: load_path.to_str().unwrap()};
        let n = 5;

        let commander = Commander::new(load_path);
        assert_eq!(
            fxt.tail(n),
            commander.tail(n)
        )
    }
}