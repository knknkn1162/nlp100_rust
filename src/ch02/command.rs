use std::process::Command;
use std::path::Path;

/// preparation for ch02; save the tab-splited file, hightemp.txt
/// The file is the record of maximum temperature in Japan.
/// these data are composed of prefecture, location, temperature and date.
pub fn prepare<P: AsRef<Path>>(save_path: P) {
    let path = "http://www.cl.ecei.tohoku.ac.jp/nlp100/data/hightemp.txt";
    let output = Command::new("curl")
        .arg(path)
        .args(&["-o", save_path.as_ref().to_str().unwrap()])
        .output().expect("fail to execute process");

    debug!("status: \n{:?}", output.status);
    debug!("stdout: \n{}", String::from_utf8_lossy(&output.stdout));
    debug!("stderr: \n{}", String::from_utf8_lossy(&output.stderr));
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
        prepare(save_path);

        assert!(save_path.exists())
    }
}