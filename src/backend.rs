extern crate diesel;
extern crate glob;
extern crate iron_diesel_middleware;
extern crate regex;
extern crate serde_derive;
extern crate serde_json;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use glob::glob;
use regex::Regex;
use std::path::Path;
use std::process::Command;

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
pub struct Video {
    id: i32,
    title: String,
    code: String,
    location: String,
    cover: String,
}

#[derive(Insertable)]
#[table_name = "videos"]
pub struct NewVideo {
    title: String,
    code: String,
    location: String,
    cover: String,
}

table! {
    videos (id) {
        id -> Integer,
        title -> Text,
        code -> Text,
        location -> Text,
        cover -> Text,
    }
}

#[derive(Identifiable, Serialize, Deserialize, Queryable, Associations)]
pub struct Actress {
    id: i32,
    name: String
}

#[derive(Insertable)]
#[table_name = "actresss"]
pub struct NewActress {
    name: String
}

table! {
    actresss (id) {
        id -> Integer,
        name -> Text,
    }
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Video)]
#[belongs_to(Actress)]
pub struct VideoActress {
    id: i32,
    video_id: i32,
    actress_id: i32
}

#[derive(Insertable)]
#[table_name = "video_actresss"]
pub struct NewVideoActress {
    video_id: i32,
    actress_id: i32
}

table! {
    video_actresss (id) {
        id -> Integer,
        video_id -> Integer,
        actress_id -> Integer,
    }
}

pub fn get_videos(conn: &SqliteConnection) -> Vec<Video> {
    videos::table.load::<Video>(conn).unwrap()
}

pub fn scan_videos(conn: &SqliteConnection) {
    diesel::delete(videos::table).execute(conn).unwrap();
    diesel::delete(actresss::table).execute(conn).unwrap();

    for entry in glob("/mnt/storage/JAV/*/* *.[!j]*").unwrap() {
        match entry {
            Ok(path) => {
                let v = create_video(&path);
                diesel::insert_into(videos::table)
                    .values(&v)
                    .execute(conn)
                    .unwrap();

                let video_stored = videos::table
                    .filter(videos::code.eq(v.code))
                    .first::<Video>(conn)
                    .unwrap();
                
                match create_actresss(&path) {
                    Some(actresss) => {
                        for a in actresss {
                            diesel::insert_or_ignore_into(actresss::table)
                                .values(&a)
                                .execute(conn)
                                .unwrap();
                            let actress_stored = actresss::table
                                .filter(actresss::name.eq(a.name))
                                .first::<Actress>(conn)
                                .unwrap();

                            let video_actress = NewVideoActress {
                                video_id: video_stored.id,
                                actress_id: actress_stored.id
                            };
                            diesel::insert_into(video_actresss::table)
                                .values(&video_actress)
                                .execute(conn)
                                .unwrap();
                        }
                    }
                    None => {}
                }
            }
            Err(_e) => {}
        }
    }
}

pub fn play_video(conn: &SqliteConnection, id: i32) {
    let video = videos::table.find(id).first::<Video>(conn).unwrap();
    Command::new("xdg-open")
        .arg(video.location.to_owned())
        .output()
        .unwrap();
}

pub fn search(conn: &SqliteConnection, code: &str, title: &str, actress: &str) -> Vec<Video> {
    let videos = videos::table
        .filter(videos::code.like(format!("%{}%", code)))
        .filter(videos::title.like(format!("%{}%", title)))
        .load::<Video>(conn)
        .unwrap();

    let actresss = actresss::table
        .filter(actresss::name.like(format!("%{}%", actress)))
        .load::<Actress>(conn)
        .unwrap();

    let mut true_videos: Vec<Video> = Vec::new();
    for video in videos {
        println!("{}", video.code);
        let video_actresss = video_actresss::table
            .filter(video_actresss::video_id.eq(video.id))
            .load::<VideoActress>(conn)
            .unwrap();
        
        let mut toggle = true;
        if video_actresss.len() != 0 {
            for actress in actresss.iter() {
                if actress.id != video_actresss[0].actress_id {
                    toggle = false;
                }
            }
        }
        if toggle {
            true_videos.push(video);
        }
    }
    return true_videos;
}

fn create_actresss(path: &Path) -> Option<Vec<NewActress>> {
    let filename = path.file_name()?.to_str()?;
    let re = Regex::new(r"\[(.*?)\]").unwrap();
    let mut iter = re.captures_iter(filename);
    iter.next()?;
    let names = iter.next()?.get(1)?.as_str().to_string();

    Some(names.split(",").map(|n| {
        NewActress {
            name: n.to_string()
        }
    }).collect())
}

fn create_video(path: &Path) -> NewVideo {
    let filename = path.file_name().unwrap().to_str().unwrap();
    let re_code = Regex::new(r"\[(?P<code>.*?)\]").unwrap();
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
        .name("code")
        .map_or("".to_string(), |m| m.as_str().to_string());
    let title = match re_title.captures(filename) {
        Some(v) => v
            .name("title")
            .map_or("".to_string(), |m| m.as_str().to_string()),
        None => "".to_string(),
    };

    NewVideo {
        title: title,
        code: code.to_owned(),
        location: String::from(path.to_str().unwrap()),
        cover: format!("{}/{} Cover Thumb.jpg", dir, code),
    }
}
