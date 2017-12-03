extern crate redis;
extern crate serde_json;

use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Artist {
    id: u32,
    gid: String,
    name: String,
    sort_name: String,
    area: Option<String>,
    aliases: Option<Vec<Alias>>,
    tags: Option<Vec<Tag>>,
    begin: Option<Date>,
    end: Option<Date>,
    rating: Option<Rating>,
}

#[derive(Debug, Deserialize)]
struct Date {
    year: Option<i32>,
    month: Option<u8>,
    date: Option<u8>,
}

#[derive(Debug, Deserialize)]
struct Alias {
    name: Option<String>,
    sort_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Tag {
    count: Option<u32>,
    #[serde(rename="value")]
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Rating {
    count: Option<u32>,
    #[serde(rename="value")]
    avg: Option<u32>,
}


use self::redis::{Commands, RedisResult};

fn create_name_area_kvs() {
    let client = redis::Client::open("redis://127.0.0.1").unwrap();
    let conn = client.get_connection().unwrap();


    let reader = BufReader::new(File::open("./data/ch07/artist.json").unwrap());
    reader.lines()
        .map(|line| serde_json::from_str::<Artist>(&line.unwrap()).unwrap())
        .for_each(|artist| {
            let _: () = conn.set(artist.name, artist.area.unwrap_or("".to_string())).unwrap();
        });

}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_name_area_kvs() {
        let res = create_name_area_kvs();
        println!("{:?}", res);
    }
}