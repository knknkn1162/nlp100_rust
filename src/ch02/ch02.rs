use std::io::{BufReader, BufRead, Read, BufWriter, Write};
use std::fs::File;
use std::io::{self, Result as ioResult};
use std::path::Path;
use std::collections::HashMap;

use super::{rw, util};

struct FileExtractor<'a> {path: &'a Path}

impl<'a> FileExtractor<'a> {
    pub fn new<P: AsRef<Path>+?Sized>(path: &P)-> FileExtractor {
        FileExtractor {path: path.as_ref()}
    }

    /// helper for read designated file. ignore error
    fn read(&self)->ioResult<String> {
        rw::read(self.path)
    }

    /// return iterator instead of String in read method.
    fn read_lines(&self)->ioResult<Vec<String>> {
        rw::read_lines(self.path)
    }

    /// ch02.10 count lines
    pub fn count_lines(&self)->usize {
        self.read_lines()
            .unwrap()
            .len()
    }

    /// ch02.11 replace a tab-character to a space
    pub fn replace_tab_to_space(&self)->String {
        self.read()
            .unwrap()
            .replace("\t", " ")
    }

    /// helper for ch02.12
    /// n: col index beginning with 0.
    fn extract_row(&self, n: usize)->Vec<String> {
        self.read_lines()
            .unwrap()
            .into_iter()
            .map(|line| {
                line.split('\t')
                    .nth(n)
                    .unwrap()
                    .to_string()
            })
            .collect()
    }

    /// ch02.12; save first and second row in each file
    pub fn save_first_second_row<T: AsRef<Path>>(&self, file1: &T, file2: &T) {
        vec![file1, file2]
            .into_iter()
            .enumerate()
            .map(|(idx, file)| {
                let v = self.extract_row(idx);
                let _ = rw::write_lines(&v, file)
                    .unwrap();
            })
            .collect::<Vec<_>>();
    }

    /// helper for ch03.13; merge col1.txt and col2.txt
    fn merge<S: AsRef<str>>(row1: &Vec<S>, row2: &Vec<S>)-> String {
        row1.iter()
            .zip(row2.iter())
            .map(|(item1, item2)|
                format!("{}{}{}", item1.as_ref(), '\t', item2.as_ref())
            )
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// ch03.13; save result of merge method.
    pub fn save_merge<T1: AsRef<Path>, T2: AsRef<Path>>(file1: T1, file2: T1, save_file: T2) {
        let lines = vec![file1, file2]
            .into_iter()
            .map(|file|
                rw::read_lines(file)
                    .unwrap()
            )
            .collect::<Vec<_>>();

        let res = FileExtractor::merge(&lines[0], &lines[1]);

        rw::write(&res, save_file);
    }

    /// ch02.14 take first ${num} lines
    pub fn head(&self, n: usize)->String {
        self.read_lines()
            .unwrap()
            .into_iter()
            .take(n)
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// ch02.15 tail last ${num} lines
    pub fn tail(&self, n: usize)->String {
        let mut v = self.read_lines()
            .unwrap()
            .into_iter()
            .rev()
            .take(n)
            .collect::<Vec<_>>();
        v.reverse();
        v.join("\n")
    }

    /// helper for ch02.16 return String
    fn split(&self, n: usize)->Vec<String> {
        let split_n = super::util::get_split_line_count(
            self.count_lines(),
            n
        );
        self.read_lines()
            .unwrap()
            .chunks(split_n)
            .map(|ws|
                ws.join("\n")
            )
            .collect()
    }

    /// ch02.16 split ${n} files
    /// return is success count of saving files.
    pub fn save_split<P: AsRef<Path>>(&self, n: usize, dst: P)->usize {
        let vs = self.split(n);
        let save_path = dst.as_ref();
        assert!(n <= 24); // assume that limit is the number of alphabet.
        let filenames =
            (b'a'..b'z').map(|s| {
                let new_filename = format!(
                    "{}{}",
                    save_path.file_name()
                        .unwrap()
                        .to_str()
                        .unwrap(),
                    format!("{}{}", 'a', s as char)
                );
                save_path.parent()
                    .unwrap()
                    .join(&new_filename)
            })
                .take(n)
                .collect::<Vec<_>>();


        // write files
        vs.iter()
            .zip(filenames.iter())
            .map(|(s, file)|
                     rw::write(s, file)
            )
            .collect::<Result<Vec<_>,_>>()
            .unwrap()
            .len()
    }

    /// ch02.17 collect unique items in first row.
    pub fn uniq_first_row(&self)->String {
        let mut lines = self.extract_row(0);
        lines.sort_unstable();
        lines.dedup();
        lines.join("\n")
    }

    /// ch02.18 sort by third columns in descending
    pub fn sort_in_descending(&self, n: usize)->Vec<String> {
        let s = self.read().unwrap();
        let delimiter = '\t';
        let mut lines = s.lines()
            .map(|line|
                     line.split(delimiter)
                         .collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

        lines.sort_by(|line1, line2| {
            let f1 = line1[n-1].parse::<f32>().unwrap();
            let f2 = line2[n-1].parse::<f32>().unwrap();
            f2.partial_cmp(&f1).unwrap() // in descending
        });

        lines.iter().map(|line| line.join(&format!("{}",delimiter))).collect()
    }

    /// ch02.19 sort by the number of prefectures listing first columns.
    pub fn sort_by_frequent_item(&self)->Vec<String> {
        // generate ordering key
        let mut hashmap = HashMap::new();
        for key in self.extract_row(0) {
            let counter = hashmap.entry(key).or_insert(0);
            *counter += 1;
        }
        let mut partial_ordering = hashmap.into_iter()
            .collect::<Vec<(String, i32)>>();

        partial_ordering.sort_by_key(|&(_, count)| -count);
        // parfect ordering
        let ordering = partial_ordering.into_iter()
            .enumerate()
            .map(|(idx, (s, _))| (s, idx))
            .collect::<HashMap<String, usize>>();
        info!("{:?}", ordering);

        let delimiter = "\t";
        let s = self.read().unwrap();
        let mut lines = s.lines()
            .map(|line|
                line.split(delimiter)
                    .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>();
        info!("{:?}", lines);
        (&mut lines).sort_by_key(|line| {
            ordering[line[0]]
        });

        lines.into_iter()
            .map(|s| s.join(delimiter)).collect()

    }


}

#[cfg(test)]
mod test {
    use ch02::command::Commander;
    use super::*;
    extern crate glob;

    #[test]
    fn test_read() {
        let fext = FileExtractor::new("./data/ch02/hightemp.txt");
        let buf = fext.read().unwrap();

        assert_eq!(
            buf.lines().take(1).collect::<String>(),
            "高知県\t江川崎\t41\t2013-08-12"
        );
    }

    #[test]
    fn test_read_lines() {
        let fxt = FileExtractor::new("./data/ch02/hightemp.txt");

        let res = fxt.read_lines();

        assert_eq!(
            res.unwrap().iter().next().unwrap(),
            "高知県\t江川崎\t41\t2013-08-12"
        )
    }

    #[test]
    fn test_ch02_10_count_lines() {
        let path = "./data/ch02/hightemp.txt";
        let fxt = FileExtractor::new(path);

        let commander = Commander::new(path);
        assert_eq!(fxt.count_lines(), commander.count_lines().unwrap())
    }

    #[test]
    fn test_ch02_11_replace_tab_to_space() {
        let path = "./data/ch02/hightemp.txt";
        let fxt = FileExtractor::new(path);

        let commander = Commander::new(path);

        assert_eq!(
            fxt.replace_tab_to_space(),
            commander.replace_tab_to_space()
        )
    }

    #[test]
    fn test_ch02_12_helper_extract_row() {
        let load_path = Path::new("./data/ch02/hightemp.txt");
        let fxt = FileExtractor::new(load_path);

        let commander = Commander::new(load_path);

        assert_eq!(
            commander.extract_row(0).lines().collect::<Vec<_>>(),
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
            .map(|fpath| ::std::fs::remove_file(fpath))
            .collect::<Vec<_>>();

        let fxt = FileExtractor::new(load_path);
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
        let fxt = FileExtractor::new(load_path);

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
        let save_file = parent.join("col_12.txt");

        let fxt = FileExtractor::new(load_path);
        fxt.save_first_second_row(&file1, &file2);

        // assume that save_file doesn't exist.
        let _ = ::std::fs::remove_file(&save_file);
        FileExtractor::save_merge(&file1, &file2, &save_file);

        assert!(save_file.exists());
    }

    #[test]
    fn test_head() {
        let load_path = "./data/ch02/hightemp.txt";
        let fxt = FileExtractor::new(load_path);
        let n = 5;

        let commander = Commander::new(load_path);
        assert_eq!(
            fxt.head(n),
            commander.head(n)
        )
    }

    #[test]
    fn test_tail() {
        let load_path = "./data/ch02/hightemp.txt";
        let fxt = FileExtractor::new(load_path);
        let n = 5;

        let commander = Commander::new(load_path);
        assert_eq!(
            fxt.tail(n),
            commander.tail(n)
        )
    }

    #[test]
    fn test_split() {
        let load_path = Path::new("./data/ch02/hightemp.txt");

        let fxt = FileExtractor::new(&load_path);

        let commander = Commander::new(load_path);

        let save_path= Path::new("./data/ch02/split_");

        let n = 3;
        commander.split(n, &save_path);

        use self::glob::glob;
        let vfs = glob("./data/ch02/split_*")
            .expect("failed to read glob pattern")
            .collect::<Result<Vec<_>,_>>()
            .unwrap();

        use std::collections::HashSet;
        let hashset = vfs.iter().map(|file| {
            rw::read(file).unwrap().trim().into()
        }).collect::<HashSet<_>>();

        let vs = fxt.split(n);

        assert_eq!(n, vs.len());

        // compare result of split command with FileExtractor::split
        assert_eq!(
            vs.into_iter().collect::<HashSet<_>>(),
            hashset
        )
    }

    #[test]
    fn test_save_split() {
        let load_path = Path::new("./data/ch02/hightemp.txt");

        let fxt = FileExtractor::new("./data/ch02/hightemp.txt");

        let n = 3;
        // check number of success in files must be equal to n
        assert_eq!(
            fxt.save_split(n, "./data/ch02/split_"),
            n
        )

    }

    #[test]
    fn test_uniq_first_row() {
        let load_path = Path::new("./data/ch02/hightemp.txt");

        let fxt = FileExtractor::new(load_path);

        let commander = Commander::new(load_path);
        assert_eq!(
            fxt.uniq_first_row(),
            commander.uniq_first_row()
        );
    }

    #[test]
    fn test_sort_in_descending() {
        let load_path = Path::new("./data/ch02/hightemp.txt");

        let fxt = FileExtractor::new(load_path);
        let commander = Commander::new(load_path);

        let n = 5;

        eprintln!("{:?}",
                 &fxt.sort_in_descending(3)[..n]
        );
        // sort command is unstable, so maybe different from sort in rust.
        eprintln!("{:?}",
                 commander.sort_in_descending(3)
                     .lines()
                     .take(n)
                     .collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_sort_by_frequent_item() {
        let fxt = FileExtractor::new("./data/ch02/hightemp.txt");

        let res = fxt.sort_by_frequent_item();

        // confirm manually
        // hashmap takes values at random!
        eprintln!("{:?}", res)
    }
}