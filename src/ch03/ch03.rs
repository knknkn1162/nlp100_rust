extern crate reqwest;
use std::io::{Read, copy, Result as ioResult};
use std::path::Path;
use std::fs::File;
use std::process::{Command, Stdio};


fn get_json<T: reqwest::IntoUrl, P: AsRef<Path>>(url: T, save_dir: P)->ioResult<()> {
    let mut response = reqwest::get(url).unwrap();
    let fname = {
        let fname = response.url()
            .path_segments()
            .unwrap()
            .last()
            .unwrap();
        save_dir.as_ref().join(fname)
    };

    // NOT use unwrap method to ignore whether save_dir exists or not
    let _ = ::std::fs::create_dir(save_dir);
    let mut f = File::create(&fname)?;
    let _ = copy(&mut response, &mut f);

    let output = Command::new("gunzip")
        .arg(fname)
        .output()?;

    eprintln!("status: {:?}", output.status);
    eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    Ok(())

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_json() {
        let dir = "./data/ch03";

        // assume json file not exist
        let _ = ::std::fs::remove_dir_all(dir);

        let _ = get_json(
            "http://www.cl.ecei.tohoku.ac.jp/nlp100/data/jawiki-country.json.gz",
        dir,
        ).unwrap();

        assert!(Path::new("./data/ch03/jawiki-country.json").exists());
    }
}