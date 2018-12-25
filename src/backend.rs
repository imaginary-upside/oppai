extern crate serde_derive;
extern crate serde_json;
extern crate glob;
extern crate regex;
extern crate rusqlite;

use std::path::Path;
use glob::glob;
use regex::Regex;
use rusqlite::{Connection, NO_PARAMS};
use std::process::Command;

#[derive(Serialize, Deserialize)]
pub struct Video {
    title: String,
    code: String,
    location: String,
}

pub fn get_videos() -> Vec<Video> {
    scan_videos();
    let conn = Connection::open("database.sqlite").unwrap();
    let mut stmt = conn.prepare("SELECT title, code, location FROM video").unwrap();
    let video_iter = stmt.query_map(NO_PARAMS, |row| Video {
        title: row.get(0),
        code: row.get(1),
        location: row.get(2)
    }).unwrap();
    video_iter.map(|video| video.unwrap()).collect()
}

pub fn scan_videos() {
    let conn = Connection::open("database.sqlite").unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS video (
            id INTEGER PRIMARY KEY,
            code TEXT,
            title TEXT,
            location TEXT
            )", NO_PARAMS).unwrap();

    conn.execute("DELETE FROM video", NO_PARAMS).unwrap();

    for entry in glob("/mnt/storage/JAV/*/* *.mkv").unwrap() {
        match entry {
            Ok(path) => {
                let video = create_video(&path);
                conn.execute("INSERT INTO video (code, title, location)
                             VALUES (?1, ?2, ?3)",
                             &[&video.code, &video.title, &video.location]).unwrap();
            },
            Err(_e) => {}
        }
    } 
}

pub fn play_video(code: &str) {
    let conn = Connection::open("database.sqlite").unwrap();
    let mut stmt = conn.prepare("SELECT title, code, location
                                FROM video
                                WHERE code = ?1
                                LIMIT 1").unwrap();
    let mut video_iter = stmt.query_map(&[code], |row| Video {
        title: row.get(0),
        code: row.get(1),
        location: row.get(2),
    }).unwrap();
    let video = video_iter.next().unwrap().unwrap();
    Command::new("xdg-open").arg(video.location).output().unwrap();
}

fn create_video(path: &Path) -> Video {
    let filename = path.file_name().unwrap().to_str().unwrap();
    let re_code = Regex::new(r"\[(?P<title>.*?)\]").unwrap();
    let re_title = Regex::new(r"\](?P<title>.*?)\[").unwrap();
    return Video {
        title: "re_code.captures(filename).unwrap()[0]".to_string(),
        code: re_code.captures(filename).unwrap().name("title").map_or("".to_string(), |m| m.as_str().to_string()),
        location: String::from(path.to_str().unwrap())
    }
}
