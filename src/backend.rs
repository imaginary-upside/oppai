extern crate glob;
extern crate regex;
extern crate serde_derive;
extern crate serde_json;
extern crate rusqlite;

use glob::glob;
use regex::Regex;
use std::path::Path;
use std::process::Command;
use crate::config::SETTINGS;
use crate::models::*;

pub fn get_videos(conn: rusqlite::Connection) -> Vec<Video> {
    let mut stmt = conn.prepare("SELECT rowid, * FROM video").unwrap();
    let video_iter = stmt.query_map(rusqlite::NO_PARAMS, map_sql_to_video).unwrap();
    video_iter.map(|video| video.unwrap()).collect()
}

pub fn scan_videos(conn: rusqlite::Connection) {
    conn.execute("delete from video", rusqlite::NO_PARAMS).unwrap();
    conn.execute("delete from actress", rusqlite::NO_PARAMS).unwrap();
    conn.execute("delete from video_actress", rusqlite::NO_PARAMS).unwrap();

    let glob_path = SETTINGS.read().unwrap().get::<String>("path").unwrap();
    for entry in glob(&glob_path).unwrap() {
        match entry {
            Ok(path) => {
                let video = match create_video(&path) {
                    Some(v) => v,
                    None => continue
                };
                conn.execute("INSERT INTO video (code, title, location, cover)
                             VALUES (?1, ?2, ?3, ?4)",
                    &[&video.code, &video.title, &video.location, &video.cover]).unwrap();


                let mut stmt_fuck = conn.prepare("select rowid, * from video where code = ?1").unwrap();
                let mut video_iter = stmt_fuck.query_map(&[&video.code], map_sql_to_video).unwrap();
                let stored_video = video_iter.next().unwrap().unwrap();
                
                match create_actresss(&path) {
                    Some(actresss) => {
                        for actress in actresss {
                            conn.execute("insert into actress (name) select ?1
                                         where not exists(select 1 from actress where name = ?1)",
                                &[&actress.name]).unwrap();


                            let mut stmt = conn.prepare("select rowid, * from actress where name = ?1").unwrap();
                            let mut actress_iter = stmt.query_map(&[&actress.name], map_sql_to_actress).unwrap();
                            let actress = actress_iter.next().unwrap().unwrap();


                            conn.execute("insert into video_actress (video_id, actress_id) values (?1, ?2)",
                                &[stored_video.id, actress.id]).unwrap();
                        }
                    },
                    None => {}
                };
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

fn create_actresss(path: &Path) -> Option<Vec<Actress>> {
    let filename = path.file_name()?.to_str()?;
    let re = Regex::new(r"\[(.*?)\]").unwrap();
    let mut iter = re.captures_iter(filename);
    iter.next()?;
    let names = iter.next()?.get(1)?.as_str().to_string();

    Some(
        names
            .split(",")
            .map(|n| Actress {
                id: None,
                name: n.to_string(),
            })
            .collect(),
    )
}

fn create_video(path: &Path) -> Option<Video> {
    let filename = path.file_name()?.to_str()?;
    let re_code = Regex::new(r"\[(?P<code>.*?)\]").unwrap();
    let re_title = Regex::new(r"\](?P<title>.*?)\[").unwrap();

    let dir: String = path.parent()?.file_name()?.to_str()?.to_string();
    let code = re_code
        .captures(filename)?
        .name("code")?
        .as_str()
        .to_string();
    let title = re_title
        .captures(filename)?
        .name("title")?
        .as_str()
        .to_string();

    Some(Video {
        id: None,
        title: title,
        code: code.to_owned(),
        location: String::from(path.to_str()?),
        cover: format!("{}/{} Cover Thumb.jpg", dir, code),
    })
}
