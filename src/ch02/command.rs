use std::process::{Command, Stdio};
use std::io::{Read, Write};
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
    pub fn count_lines(&self)->Result<usize, ::std::num::ParseIntError> {
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

    /// ch02_11; replace tab to space
    pub fn replace_tab_to_space(&self)->String {
        let mut cat = Command::new("cat")
            .arg(&self.path)
            .stdout(Stdio::piped())
            .spawn().expect("fail to execute cat command");
        let mut tr = Command::new("tr")
            .arg("[:blank:]")
            .arg(" ")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn().expect("fail to execute tr command");
        // see https://www.reddit.com/r/rust/comments/3azfie/how_to_pipe_one_process_into_another/
        if let Some(ref mut stdout) = cat.stdout {
            if let Some(ref mut stdin) = tr.stdin {
                let mut buf: Vec<u8> = Vec::new();
                stdout.read_to_end(&mut buf).unwrap();
                stdin.write_all(&buf).unwrap();

            }
        }
        let res = tr.wait_with_output().unwrap().stdout;
        String::from_utf8(res).expect("contain invalid utf-8 character")
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

        // Success or not, ignore result
        // see also https://github.com/rust-lang/rust/pull/11754#issuecomment-33202664
        let _ = fs::create_dir(save_path.parent().unwrap());
        let commander = Commander::new(save_path);
        commander.prepare();

        assert!(save_path.exists())
    }

    #[test]
    fn test_count_lines() {
        let save_path = Path::new("./data/ch02/hightemp.txt");
        let commander = Commander::new(save_path);

        assert_eq!(commander.count_lines().unwrap(), 24);
    }

    #[test]
    fn test_replace_tab_to_space() {
        let save_path = Path::new("./data/ch02/hightemp.txt");
        let commander = Commander::new(save_path);
        let res = commander.replace_tab_to_space();

        assert_eq!(
            res.lines().take(1).collect::<String>(),
            "高知県 江川崎 41 2013-08-12"
        )

    }
}