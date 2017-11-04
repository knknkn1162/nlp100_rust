use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead, Read, Write}; // Read is used for read_to_string
use std::fs::File;
use std::path::Path;

pub struct Commander {path: String}

impl Commander {
    pub fn new<P: AsRef<Path>>(save_path: P) -> Commander {
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
    pub fn count_lines(&self) -> Result<usize, ::std::num::ParseIntError> {
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
    pub fn replace_tab_to_space(&self) -> String {
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

    /// preparation to ch02_12
    pub fn extract_row(&self, n: usize) -> String {
        let res = Command::new("cut")
            .args(&["-f", &format!("{}", n + 1)]) // start at 0
            .arg(&self.path)
            .output().expect("fail to execute cut command");

        String::from_utf8_lossy(&res.stdout).trim().to_string()
    }

    /// ch02.13 merge 2 files
    pub fn merge<P: AsRef<Path>>(file1: &P, file2: &P)->String {
        let res = Command::new("paste")
            .args(&[file1.as_ref(), file2.as_ref()])
            .output().expect("fail to execute paste command");

        String::from_utf8_lossy(&res.stdout).trim().to_string()
    }

    /// helper for ch02. 14&15
    fn take(&self, n: usize, pos: &str)->String {
        let res = Command::new(pos)
            .args(&["-n", format!("{}", n).as_str()])
            .arg(&self.path)
            .output().expect("fail to execute head command");

        String::from_utf8_lossy(&res.stdout).trim().to_string()
    }

    /// ch02.14 `head -n ${file}`
    pub fn head(&self, n: usize)->String {
        self.take(n, "head")
    }

    /// ch02.15 `tail -n ${file}
    pub fn tail(&self, n: usize)->String {
        self.take(n, "tail")
    }

    /// ch02.16 split n files.
    pub fn split<P: AsRef<Path>>(&self, n: usize, dst: &P) {
        let size = self.count_lines().unwrap();
        use ch02::util;
        let lines = util::get_split_line_count(size, n);
        debug!("split per {} lines", lines);
        assert!(lines >0);
        let res = Command::new("split")
            .args(&["-l", &format!("{}", lines)])
            .arg(&self.path) // src
            .arg(dst.as_ref().to_str().unwrap()) // dst
            .output()
            .expect("fail to execute split command");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate env_logger;
    extern crate getopts;
    extern crate glob;

    use self::getopts::Options;
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

    #[test]
    fn test_extract_row() {
        let load_path = Path::new("./data/ch02/hightemp.txt");
        let commander = Commander::new(load_path);
        assert_eq!(
            commander.extract_row(0).lines().next().unwrap(), // take first line
            "高知県"
        );
    }

    #[test]
    fn test_merge() {
        let load_path = Path::new("./data/ch02/hightemp.txt");

        let parent = load_path.parent().unwrap();

        let file1 = parent.join("col1.txt");
        let file2 = parent.join("col2.txt");

        let res = Commander::merge(&file1, &file2);
        debug!("{:?}", res);
        assert_eq!(
            (&mut res.lines()).next().unwrap(),
            "高知県\t江川崎"
        )
    }

    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} FILE [options]", program);
        print!("{}", opts.usage(&brief));
    }

    /// with cargo test -- [<OPTIONS>], there seems to be panicked at '"Unrecognized option: \'n\'."'
    /// so set args directly instead of using env::args()
    #[test]
    fn test_head() {
        // let args = env::args()::collect::<Vec<String>>();
        let args = vec!["program", "-n", "5", "./data/ch02/hightemp.txt"];

        let program = args[0].clone();

        let mut opts = Options::new();
        opts.optopt("n", "num", "set first ${num} rows", "NUMBER");
        opts.optflag("h", "help", "print this help menu");

        let matches = opts.parse(&args[1..]).unwrap();

        if matches.opt_present("h") {
            print_usage(&program, opts);
            return;
        }

        let n = matches
            .opt_str("n")
            .expect("invalid number")
            .parse::<usize>()
            .unwrap();
        let input = matches.free.first().unwrap();

        let commander = Commander::new(input);

        let res = commander.head(n);
        assert_eq!(
            res,
            "高知県\t江川崎\t41\t2013-08-12\n埼玉県\t熊谷\t40.9\t2007-08-16\n\
            岐阜県\t多治見\t40.9\t2007-08-16\n山形県\t山形\t40.8\t1933-07-25\n\
            山梨県\t甲府\t40.7\t2013-08-10"
        );
    }

    #[test]
    fn test_tail() {
        // let args = env::args()::collect::<Vec<String>>();
        let args = vec!["program", "-n", "5", "./data/ch02/hightemp.txt"];

        let program = args[0].clone();

        let mut opts = Options::new();
        opts.optopt("n", "num", "set first ${num} rows", "NUMBER");
        opts.optflag("h", "help", "print this help menu");

        let matches = opts.parse(&args[1..]).unwrap();

        if matches.opt_present("h") {
            print_usage(&program, opts);
            return;
        }

        let n = matches
            .opt_str("n")
            .expect("invalid number")
            .parse::<usize>()
            .unwrap();
        let input = matches.free.first().unwrap();

        let commander = Commander::new(input);

        let res = commander.tail(n);
        println!("{:?}", res);
        assert_eq!(
            res,
            "埼玉県\t鳩山\t39.9\t1997-07-05\n\
            大阪府\t豊中\t39.9\t1994-08-08\n\
            山梨県\t大月\t39.9\t1990-07-19\n\
            山形県\t鶴岡\t39.9\t1978-08-03\n\
            愛知県\t名古屋\t39.9\t1942-08-02"
        );
    }

    #[test]
    fn test_split() {
        let args = vec!["program", "--line", "3", "./data/ch02/hightemp.txt", "./data/ch02/split_"];

        let program = args[0].clone();


        let mut opts = Options::new();
        opts.optopt("l", "line", "set first ${num} rows", "NUMBER");
        opts.optflag("h", "help", "print this help menu");

        let matches = opts.parse(&args[1..]).unwrap();

        if matches.opt_present("h") {
            print_usage(&program, opts);
            return;
        }

        let split_num = matches
            .opt_str("l")
            .unwrap()
            .parse::<usize>()
            .expect("invalid number");

        let input = &matches.free[0..2];

        let save_path = Path::new(&input[1]);

        let commander = Commander::new(&input[0]);

        //
        commander.split(split_num, &input[1]);
        let filename = format!("{}{}", save_path.file_name().unwrap().to_str().unwrap(), '*');

        use self::glob::glob;

        // check that all ok and the length of vector is equal to split_num
        assert_eq!(
            glob(save_path.parent().unwrap().join(&filename).to_str().unwrap())
                .expect("failed to read glob pattern")
                .collect::<Result<Vec<_>,_>>().unwrap().len(),
            split_num
        );
    }
}