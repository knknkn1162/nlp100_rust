extern crate reqwest;
extern crate serde_json;
use self::serde_json::Result as jsonResult;
use std::io::{BufReader, BufRead, copy, Result as ioResult};
use std::path::Path;
use std::fs::File;
use std::process::{Command, Stdio};


#[derive(Serialize, Deserialize)]
struct Article {
    title: String,
    text: String,
}

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
        .arg(&fname)
        .output()?;

    eprintln!("status: {:?}", output.status);
    eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    Ok(())
}

struct JsonExtractor<'a> {
    path: &'a Path,
}

impl<'a> JsonExtractor<'a> {
    fn new<P: AsRef<Path>+?Sized>(path: &P)->JsonExtractor {
        JsonExtractor {path: path.as_ref()}
    }



    /// helper for ch03.20; search designated article
    fn search(&self, title: &str)->Option<Article> {
        let reader = BufReader::new(File::open(self.path).unwrap());
        reader.lines()
            .flat_map(|line| serde_json::from_str::<Article>(&line.unwrap()))
            .find(|line| line.title == title)
    }


    /// ch03.21 extract text.
    pub fn extract_text(&self, title: &str)->String {
        self.search(title)
            .unwrap()
            .text
    }

    /// ch03.22 extract Category lines that startswith [[Category:
    pub fn extract_categories(&self, title: &str)->Vec<String> {
        self.extract_text(title)
        .lines()
            .filter_map(|line| if line.starts_with("[[Category") {Some(line.into())} else {None})
            .collect()
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_json() {
        let dir = "./data/ch03";

        // assume json file not exist
        let _ = ::std::fs::remove_dir_all(dir);

        get_json(
            "http://www.cl.ecei.tohoku.ac.jp/nlp100/data/jawiki-country.json.gz",
        dir,
        ).unwrap();

        assert!(Path::new("./data/ch03/jawiki-country.json").exists());
    }

    #[test]
    fn test_search() {
        let ext = JsonExtractor::new("./data/ch03/jawiki-country.json");
        let key = "イギリス";
        let res = ext.search(key);

        assert_eq!(res.unwrap().title, key);
    }

    #[test]
    fn test_extract_text() {
        let ext = JsonExtractor::new("./data/ch03/jawiki-country.json");
        let key = "イギリス";
        let res = ext.extract_text(key);

        assert_eq!(res.lines().next().unwrap(), "{{redirect|UK}}gi");
    }

}