use std::process::Command;
use std::path::Path;

pub struct Commander {path: String}

impl Commander {

    pub fn new<P: AsRef<Path>>(save_path: P)->Commander {
        Commander {
            path: save_path.as_ref().to_str().expect("contains invalid utf-8 character").to_owned()
        }
    }

    /// preparation for ch02; save the tab-splited file, hightemp.txt
    /// exec `curl http://www.cl.ecei.tohoku.ac.jp/nlp100/data/hightemp.txt" -o ${save_path}`
    /// The file is the record of maximum temperature in Japan.
    /// these data are composed of prefecture, location, temperature and date.
    pub fn prepare(&self) {
        let path = "http://www.cl.ecei.tohoku.ac.jp/nlp100/data/hightemp.txt";
        let output = Command::new("curl")
            .arg(path)
            .args(&["-o", &self.path])
            .output().expect("fail to execute process");

        debug!("status: {:?}", output.status);
        debug!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        debug!("stderr: \n{}", String::from_utf8_lossy(&output.stderr));
    }

    /// test ch02_10; count lines in the designated file.
    pub fn get_lines(&self)->Result<usize, ::std::num::ParseIntError> {
        let output = Command::new("wc")
            .arg("-l")
            .arg(&self.path)
            .output().expect("fail to execute process");

        String::from_utf8_lossy(&output.stdout)
            .as_ref()
            .trim()
            .split(" ")
            .take(1)
            .collect::<String>()
            .parse::<usize>()

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate env_logger;

    /// env_logger output is controlled by RUST_LOG environmental variable
    /// to debug only to this module, set `RUST_LOG=natural_lang::ch02::command=debug` in Environment variable.
    /// before save file, confirm existance in file or create dir in fs::create_dir method.
    /// create_dir method is equivalent to `mkdir -p` in unix command
    #[test]
    fn test_prepare() {
        use std::fs;
        env_logger::init().unwrap();
        let save_path = Path::new("./data/ch02/hightemp.txt");

        fs::create_dir(save_path.parent().unwrap()); // If success or not, ignore result
        let commander = Commander::new(save_path);
        commander.prepare();

        assert!(save_path.exists())
    }

    #[test]
    fn test_get_lines() {
        let save_path = Path::new("./data/ch02/hightemp.txt");
        let commander = Commander::new(save_path);

        assert_eq!(commander.get_lines().unwrap(), 24);

    }
}