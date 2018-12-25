extern crate diesel;
extern crate glob;
extern crate iron_diesel_middleware;
extern crate regex;
extern crate serde_derive;
extern crate serde_json;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use glob::glob;
use iron_diesel_middleware::{DieselMiddleware, DieselReqExt};
use regex::Regex;
use std::path::Path;
use std::process::Command;
use diesel::{sql_query, insert_into};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Video {
    id: i32,
    title: String,
    code: String,
    location: String,
    cover: String,
}

#[derive(Insertable)]
#[table_name = "video"]
pub struct NewVideo {
    title: String,
    code: String,
    location: String,
    cover: String,
}

table! {
    video (id) {
        id -> Integer,
        title -> Text,
        code -> Text,
        location -> Text,
        cover -> Text,
    }
}

pub fn get_videos(conn: &SqliteConnection) -> Vec<Video> {
    video::table.load::<Video>(conn).unwrap()
}

pub fn scan_videos(conn: &SqliteConnection) {
   /* conn.execute(
        "CREATE TABLE IF NOT EXISTS video (
            id INTEGER PRIMARY KEY,
            code TEXT,
            title TEXT,
            location TEXT,
            cover TEXT
            )",
        NO_PARAMS,
    )
    .unwrap();*/

    diesel::delete(video::table).execute(conn).unwrap();
    
    for entry in glob("/mnt/storage/JAV/*/* *.mkv").unwrap() {
        match entry {
            Ok(path) => {
                let v = create_video(&path);
                diesel::insert_into(video::table).values(&v).execute(conn).unwrap();
            }
            Err(_e) => {}
        }
    }
}

pub fn play_video(conn: &SqliteConnection, id: i32) {
    let video = video::table.find(id).first::<Video>(conn).unwrap();
    Command::new("xdg-open")
        .arg(video.location.to_owned())
        .output()
        .unwrap();
}

pub fn search(conn: &SqliteConnection, text: &str) -> Vec<Video> {
    video::table.load::<Video>(conn).unwrap()
}

fn create_video(path: &Path) -> NewVideo {
    let filename = path.file_name().unwrap().to_str().unwrap();
    let re_code = Regex::new(r"\[(?P<title>.*?)\]").unwrap();
    let re_title = Regex::new(r"\](?P<title>.*?)\[").unwrap();

    let dir: String = path
        .parent()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let code = re_code
        .captures(filename)
        .unwrap()
        .name("title")
        .map_or("".to_string(), |m| m.as_str().to_string());

    return NewVideo {
        title: "re_code.captures(filename).unwrap()[0]".to_string(),
        code: code.to_owned(),
        location: String::from(path.to_str().unwrap()),
        cover: format!("{}/{} Cover.jpg", dir, code),
    };
}
