extern crate reqwest;
use std::io::{Read, copy, Result as ioResult};
use std::path::Path;
use std::fs::File;
use std::process::{Command, Stdio};


fn download<T: reqwest::IntoUrl, P: AsRef<Path>>(url: T, save_dir: P)->ioResult<()> {
    let mut response = reqwest::get(url).unwrap();
    let fname = {
        let fname = response.url()
            .path_segments()
            .unwrap()
            .last()
            .unwrap();
        save_dir.as_ref().join(fname)
    };

    let _ = ::std::fs::create_dir(save_dir); // ignore save_dir exists
    let mut f = File::create(&fname)?;
    copy(&mut response, &mut f);

    let output = Command::new("gunzip")
        .arg(fname)
        .output()?;

    eprintln!("status: {:?}", output.status);
    eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: \n{}", String::from_utf8_lossy(&output.stderr));
    Ok(())

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_download() {
        let _ = download(
            "http://www.cl.ecei.tohoku.ac.jp/nlp100/data/jawiki-country.json.gz",
        "./data/ch03",
        );
    }
}