extern crate reqwest;
use std::io::{Read, copy};
use std::path::Path;
use std::fs::File;


fn download<T: reqwest::IntoUrl, P: AsRef<Path>>(url: T, save_dir: P) {
    let mut response = reqwest::get(url).unwrap();
    let fname = {
        let fname = response.url()
            .path_segments()
            .unwrap()
            .last()
            .unwrap();
        save_dir.as_ref().join(fname)
    };

    let _ = ::std::fs::create_dir(save_dir).unwrap();
    copy(&mut response, &mut File::create(fname).unwrap()).unwrap();
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