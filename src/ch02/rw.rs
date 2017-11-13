use std::io::{BufReader, BufRead, Read, BufWriter, Write};
use std::fs::File;
use std::io::{self, Result as ioResult};
use std::path::Path;

pub fn read<P: AsRef<Path>>(load_path: P)-> ioResult<String> {

    let mut reader = BufReader::new(File::open(load_path.as_ref()).unwrap());
    let mut buf = String::new();
    let _ = reader.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn read_lines<P: AsRef<Path>>(load_path: P)->ioResult<Vec<String>> {
    let f = File::open(load_path.as_ref())?;
    BufReader::new(f)
        .lines()
        .collect()
}

pub fn write<P: AsRef<Path>, Q: AsRef<[u8]>>(s: Q, save_path: P)->ioResult<()> {
    let f = File::create(save_path.as_ref())?;
    BufWriter::new(f)
        .write_all(s.as_ref())
}

/// use Borrow trait instead of AsRef because of join method, https://doc.rust-lang.org/std/primitive.slice.html#method.join
pub fn write_lines<P: AsRef<Path>, S: ::std::borrow::Borrow<str>>(lines: &Vec<S>, save_path: P)->ioResult<()> {
    write(lines.join("\n"), save_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    #[test]
    fn test_read() {
        let load_path: &str = "./data/ch02/sample.txt";
        let buf = "abc\nあああ\nbcd\n";
        assert_eq!(
            self::read(load_path).unwrap(),
            buf
        );

        let load_path: String = "./data/ch02/sample.txt".to_string();

        assert_eq!(
            self::read(&load_path).unwrap(),
            buf
        );

        assert_eq!(
            self::read(load_path).unwrap(), // moved
            buf
        );

        let load_path: PathBuf = PathBuf::from("./data/ch02/sample.txt");

        assert_eq!(
            self::read(&load_path).unwrap(),
            buf
        )
    }

    #[test]
    fn test_read_lines() {
        let load_path: &str = "./data/ch02/sample.txt";
        let buf = "abc\nあああ\nbcd\n".lines().collect::<Vec<_>>();

        assert_eq!(
            self::read_lines(load_path).unwrap(), buf
        )
    }

    #[test]
    fn test_write() {
        let save_path: &str = "./data/ch02/out.txt";

        let buf = "abc\nあああ\nbcd\n".to_string();

        assert_eq!(
            self::write(&buf, save_path).unwrap(), ()
        );

        assert_eq!(
            self::write(buf, save_path).unwrap(), ()
        );
    }

    #[test]
    fn test_write_lines() {
        let save_path: &str = "./data/ch02/out.txt";
        let buf = "abc\nあああ\nbcd\n".lines().collect::<Vec<_>>();

        assert_eq!(
            self::write_lines(&buf, save_path).unwrap(), ()
        );
        let s = "abc\nあああ\nbcd\n".to_string();
        let buf = s.lines().collect::<Vec<_>>();
        assert_eq!(
            self::write_lines(&buf, save_path).unwrap(), ()
        );
    }
}