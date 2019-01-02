extern crate glob;
extern crate regex;
extern crate serde_derive;
extern crate serde_json;
extern crate rusqlite;

use glob::glob;
use std::process::Command;
use crate::config::SETTINGS;
use crate::models::*;
use std::fs;

#[derive(Deserialize)]
struct VideoConfig {
    code: String,
    title: String,
    location: String,
    cover: String,
    cast: Vec<String>
}

pub fn get_videos(conn: rusqlite::Connection) -> Vec<Video> {
    let mut stmt = conn.prepare("SELECT rowid, * FROM video").unwrap();
    let video_iter = stmt.query_map(rusqlite::NO_PARAMS, map_sql_to_video).unwrap();
    video_iter.map(|video| video.unwrap()).collect()
}

pub fn scan_videos(conn: rusqlite::Connection) {
    conn.execute("delete from video", rusqlite::NO_PARAMS).unwrap();
    conn.execute("delete from actress", rusqlite::NO_PARAMS).unwrap();
    conn.execute("delete from video_actress", rusqlite::NO_PARAMS).unwrap();

    let path = SETTINGS.read().unwrap().get::<String>("path").unwrap();
    for entry in glob(&(path + "/*.json")).unwrap() {
        match entry {
            Ok(path) => {
                let data = fs::read_to_string(path).expect("unable to read file");
                let video: VideoConfig = serde_json::from_str(&data).expect("could not decode json");
                conn.execute("INSERT INTO video (code, title, location, cover)
                             VALUES (?1, ?2, ?3, ?4)",
                    &[&video.code, &video.title, &video.location, &video.cover]).unwrap();
                
                let mut stmt_fuck = conn.prepare("select rowid, * from video where code = ?1").unwrap();
                let mut video_iter = stmt_fuck.query_map(&[&video.code], map_sql_to_video).unwrap();
                let stored_video = video_iter.next().unwrap().unwrap();

                for actress in video.cast {
                    conn.execute("insert into actress (name) select ?1
                                 where not exists(select 1 from actress where name = ?1)",
                        &[&actress]).unwrap();


                    let mut stmt = conn.prepare("select rowid, * from actress where name = ?1").unwrap();
                    let mut actress_iter = stmt.query_map(&[&actress], map_sql_to_actress).unwrap();
                    let actress_stored = actress_iter.next().unwrap().unwrap();


                    conn.execute("insert into video_actress (video_id, actress_id) values (?1, ?2)",
                        &[stored_video.id, actress_stored.id]).unwrap();
                }
                
            },
            Err(_e) => {}
        }
    }
}

pub fn play_video(conn: rusqlite::Connection, id: i32) {
    let mut stmt = conn.prepare("select rowid, * from video where rowid = ?1").unwrap();
    let mut video_iter = stmt.query_map(&[id], map_sql_to_video).unwrap();
    let video: Video = video_iter.next().unwrap().unwrap();
    Command::new("xdg-open")
        .arg(video.location.to_owned())
        .output()
        .unwrap();
}

pub fn search(conn: rusqlite::Connection, video_text: &str, actress_text: &str) -> Vec<Video> {
    if video_text != "" && actress_text != "" {
        let mut stmt = conn.prepare(
            "select video.rowid, video.* from video
            join video_actress on video_actress.video_id = video.rowid
            where video_actress.actress_id in (select rowid from actress where name match ?2)
            and video match ?1").unwrap();
        let video_iter = stmt.query_map(&[video_text, actress_text], map_sql_to_video).unwrap();
        video_iter.map(|video| video.unwrap()).collect()
    } else if video_text != "" && actress_text == "" {
        let mut stmt = conn.prepare("select rowid, * from video where video match ?1").unwrap();
        let video_iter = stmt.query_map(&[video_text], map_sql_to_video).unwrap();
        video_iter.map(|video| video.unwrap()).collect()
    } else if video_text == "" && actress_text != "" {
        let mut stmt = conn.prepare(
            "select video.rowid, video.* from video
            join video_actress on video_actress.video_id = video.rowid
            where video_actress.actress_id in (select rowid from actress where name match ?1)"
        ).unwrap();
        let video_iter = stmt.query_map(&[actress_text], map_sql_to_video).unwrap();
        video_iter.map(|video| video.unwrap()).collect()
    } else {
        let mut stmt = conn.prepare("select rowid, * from video").unwrap();
        let video_iter = stmt.query_map(rusqlite::NO_PARAMS, map_sql_to_video).unwrap();
        video_iter.map(|video| video.unwrap()).collect()
    }
}
