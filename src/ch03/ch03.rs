extern crate reqwest;
extern crate serde_json;
extern crate regex;
extern crate url;
use self::serde_json::{Value, Result as jsonResult};
use std::io::{BufReader, BufRead, Read, copy, Result as ioResult};
use std::path::Path;
use std::fs::File;
use std::process::{Command, Stdio};
use self::regex::{Regex, RegexBuilder};
use std::collections::HashMap;
use self::url::Url;


#[derive(Serialize, Deserialize)]
struct Article {
    title: String,
    text: String,
}

#[derive(Debug, PartialEq)]
struct Section {
    name: String,
    level: u8,
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


    /// ch03.20 extract text.
    pub fn extract_text(&self, title: &str)->String {
        self.search(title)
            .unwrap()
            .text
    }

    /// ch03.21 extract Category lines that startswith [[Category:
    pub fn extract_categories(&self, title: &str)->Vec<String> {
        self.extract_text(title)
        .lines()
            .filter_map(|line|
                if line.starts_with("[[Category") {Some(line.into())} else {None}
            )
            .collect()
    }

    /// ch03.22 extract category names
    pub fn extract_category_names(&self, title: &str)->Vec<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[\[Category:(.*)\]\]").unwrap();
        }
        let text = self.extract_text(title);
        RE.captures_iter(&text)
            .filter_map(|caps| caps.get(1))
            .map(|s| s.as_str().into())
            .collect()
    }

    /// ch03.23 structure of section
    /// see also https://en.wikipedia.org/wiki/Wikipedia:Manual_of_Style#Section_headings
    pub fn extract_section(&self, title: &str)->Vec<Section> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<start_section>={2,})(?P<name>.*?)(?P<end_section>={2,})").unwrap();
        }
        let text = self.extract_text(title);
        RE.captures_iter(&text)
            .filter_map(|caps|
                if &caps["start_section"] == &caps["end_section"] {
                    Some(Section {
                        level: (caps["start_section"].len() - 1) as u8,
                        name: caps["name"].into(),
                    })
                } else {None}
            ).collect()
    }

    /// ch03.24 extract media file
    /// see also https://ja.wikipedia.org/wiki/Help:画像の表示
    pub fn extract_media_file(&self, title: &str)->Vec<String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[\[(?:File|ファイル):(.*?)\|").unwrap();
        }

        let text = self.extract_text(title);
        RE.captures_iter(&text)
            .filter_map(|caps| caps.get(1).map(|s| s.as_str().to_string()))
            .collect()
    }

    /// helper for ch03.25 extract template namespace
    pub fn extract_template_txt(&self, title: &str)->String{
        self.extract_text(title)
            .lines()
            .skip_while(|&s|s.starts_with("{{基本情報"))
            .take_while(|&s| s != "}}")
            .collect::<Vec<&str>>().join("\n")
    }

    /// ch03.25: extract template namespace & return as hashmap
    pub fn extract_template_map(&self, title: &str)->HashMap<String, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\|(?P<field>.*?)=(?P<value>.*?)(?:\n|<ref)").unwrap();
        }
        let text = self.extract_template_txt(title);
        RE.captures_iter(&text)
            .map(|caps| (caps["field"].trim().into(), caps["value"].trim().into()))
            .collect()
    }

    /// ch03.26 In addition to ch03.25, remove the emphasize tag
    pub fn extract_template_map_removed_em(&self, title: &str)->HashMap<String, String>  {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\|(?P<field>.*?)=(?P<value>.*?)(?:\n|<ref)").unwrap();
        }
        let text = self.extract_template_txt(title);
        RE.captures_iter(&text)
            .map(|caps| (caps["field"].trim().into(), caps["value"].trim().replace(r#"'''"#, "").replace(r"''", "")))
            .collect()
    }

    /// ch03.27 In addition to ch03.26, internal link
    pub fn extract_template_map_removed_internal(&self, title: &str)->HashMap<String, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\|(?P<field>.*?)=(?P<value>.*?)(?:\n|<ref)").unwrap();
            static ref INTER_RE: Regex = Regex::new(r"\[\[([^\[\]]*)\]\]").unwrap();
        }
        let text = self.extract_template_txt(title);

        RE.captures_iter(&text)
            .map(|caps| {
                let value = caps["value"].trim();
                let value = INTER_RE.find_iter(value)
                    .fold(value.to_string(), |acc, x|
                        acc.replace(
                            x.as_str(),
                            x.as_str().trim_matches(|c| c == '[' || c == ']').split('|').last().unwrap()
                        )
                    );
                (caps["field"].trim().into(), value.trim().replace(r#"'''"#, "").replace(r"''", ""))
            }).collect()
    }

    /// ch03.28 shape template namespace completely
    pub fn shape_template(&self, title: &str)->HashMap<String, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\{\{([^{}]*)\}\}").unwrap();
            static ref BR_RE: Regex = Regex::new(r"<br ?/?>").unwrap();
        }
        // remove <br \> or <br>
        self.extract_template_map_removed_internal(title)
            .into_iter()
            .map(|(key, value)| {
                let value = RE.find_iter(&value)
                    .fold(value.clone(), |acc, x|
                        acc.replace(
                            x.as_str(),
                            x.as_str().trim_matches(|c| c == '{' || c == '}')
                                .split('|')
                                .last()
                                .unwrap()
                        )
                    );
                (key, BR_RE.replace(&value, "").into())
            }).collect()
    }

    /// ch03.29 get flag url from Mediawiki
    /// get response to https://ja.wikipedia.org/w/api.php?action=query&titles=File:Flag%20of%20the%20United%20Kingdom%2esvg&prop=imageinfo&iiprop=url
    pub fn get_flag_url(&self)->String {
        let fname = format!("File:{}", &self.shape_template("イギリス")["国旗画像"]);

        let url = Url::parse_with_params(
            "https://ja.wikipedia.org/w/api.php",
            &[
                ("action", "query"),
                ("titles", &fname),
                ("prop", "imageinfo"),
                ("iiprop", "url"),
                ("format", "json"),
            ]
        ).unwrap();

        let json: Value = reqwest::get(url)
            .unwrap()
            .json()
            .unwrap();
        json["query"]["pages"]["-1"]["imageinfo"][0]["url"].as_str()
            .unwrap()
            .into()
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

        assert_eq!(res.lines().next().unwrap(), "{{redirect|UK}}");

        // res.lines().for_each(|s| println!("{:?}", s));
    }

    #[test]
    fn test_extract_categories() {
        let ext = JsonExtractor::new("./data/ch03/jawiki-country.json");
        let key = "イギリス";
        let res = ext.extract_categories(key);

        assert_eq!(
            res,
            vec![
                "[[Category:イギリス|*]]",
                "[[Category:英連邦王国|*]]",
                "[[Category:G8加盟国]]",
                "[[Category:欧州連合加盟国]]",
                "[[Category:海洋国家]]",
                "[[Category:君主国]]",
                "[[Category:島国|くれいとふりてん]]",
                "[[Category:1801年に設立された州・地域]]"
            ]
        )
    }

    #[test]
    fn test_extract_category_names() {
        let ext = JsonExtractor::new("./data/ch03/jawiki-country.json");
        let key = "イギリス";
        let res = ext.extract_category_names(key);

        assert_eq!(
            res,
            vec!["イギリス|*",
                 "英連邦王国|*",
                 "G8加盟国",
                 "欧州連合加盟国",
                 "海洋国家",
                 "君主国",
                 "島国|くれいとふりてん",
                 "1801年に設立された州・地域"
            ]
        )
    }

    #[test]
    fn test_extract_section() {
        let ext = JsonExtractor::new("./data/ch03/jawiki-country.json");
        let key = "イギリス";
        let res = ext.extract_section(key);

        res.iter()
            .for_each(|section| assert!(section.level>=1));

        assert_eq!(
            res.into_iter().take(5).collect::<Vec<_>>(),
            vec![
                Section{name:"国名".into(), level: 1},
                Section{name:"歴史".into(), level: 1},
                Section{name:"地理".into(), level: 1},
                Section{name: "気候".into(), level: 2},
                Section{name: "政治".into(), level: 1},
            ]
        )
    }

    #[test]
    fn test_extract_media_file() {
        let ext = JsonExtractor::new("./data/ch03/jawiki-country.json");
        let key = "イギリス";
        let res = ext.extract_media_file(key);

        assert_eq!(
            res.into_iter().take(5).collect::<Vec<_>>(),
            vec!["Royal Coat of Arms of the United Kingdom.svg",
                 "Battle of Waterloo 1815.PNG",
                 "The British Empire.png",
                 "Uk topo en.jpg",
                 "BenNevis2005.jpg",
            ]
        );
    }

    #[test]
    fn test_extract_template_map() {
        let ext = JsonExtractor::new("./data/ch03/jawiki-country.json");
        let key = "イギリス";
        let res = ext.extract_template_map(key);

        res.iter().for_each(|s| println!("{:?}", s));

        assert_eq!(
            res["標語"], "{{lang|fr|Dieu et mon droit}}<br/>（[[フランス語]]:神と私の権利）"
        );

        assert_eq!(res["公式国名"], "{{lang|en|United Kingdom of Great Britain and Northern Ireland}}");
        assert_eq!(res["GDP値MER"], "2兆4337億");
    }

    #[test]
    fn test_extract_template_map_removed_em() {
        let ext = JsonExtractor::new("./data/ch03/jawiki-country.json");
        let key = "イギリス";
        let res = ext.extract_template_map_removed_em(key);

        assert_eq!(
            res["確立形態4"], "現在の国号「グレートブリテン及び北アイルランド連合王国」に変更"
        );
    }

    #[test]
    fn test_extract_template_map_removed_internal() {
        let ext = JsonExtractor::new("./data/ch03/jawiki-country.json");
        let key = "イギリス";
        let res = ext.extract_template_map_removed_internal(key);

        // res.iter().for_each(|s| println!("{:?}", s));

        assert_eq!(
            res["標語"], "{{lang|fr|Dieu et mon droit}}<br/>（フランス語:神と私の権利）"
        );

        assert_eq!(
            res["国歌"], "神よ女王陛下を守り給え"
        );

        assert_eq!(
            res["確立形態1"],
            "イングランド王国／スコットランド王国<br />（両国とも1707年連合法まで）"
        )
    }

    #[test]
    fn test_shape_template() {
        let ext = JsonExtractor::new("./data/ch03/jawiki-country.json");
        let key = "イギリス";
        let res = ext.shape_template(key);

        assert_eq!(
            res["標語"], "Dieu et mon droit（フランス語:神と私の権利）"
        );

        assert_eq!(
            res["国歌"], "神よ女王陛下を守り給え"
        );

        assert_eq!(
            res["確立形態1"],
            "イングランド王国／スコットランド王国（両国とも1707年連合法まで）"
        );

    }

    #[test]
    fn test_get_flag_url() {
        let ext = JsonExtractor::new("./data/ch03/jawiki-country.json");
        // query is https://ja.wikipedia.org/w/api.php?action=query&titles=File%3AFlag+of+the+United+Kingdom.svg&prop=imageinfo&iiprop=url&format=json
        let res = ext.get_flag_url();

        assert_eq!(
            res, "https://upload.wikimedia.org/wikipedia/commons/a/ae/Flag_of_the_United_Kingdom.svg"
        )
    }


}