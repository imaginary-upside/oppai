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
use crate::error::Error;

#[derive(Deserialize)]
struct VideoConfig {
    code: String,
    title: String,
    location: String,
    cover: String,
    cast: Vec<String>
}

pub fn get_videos(conn: rusqlite::Connection) -> Result<Vec<Video>, Error> {
    let mut stmt = conn.prepare("SELECT rowid, * FROM video")?;
    let video_iter = stmt.query_map(rusqlite::NO_PARAMS, map_sql_to_video)?;
    Ok(video_iter.map(|video| video.unwrap()).collect())
}

pub fn scan_videos(conn: rusqlite::Connection) -> Result<(), Error> {
    conn.execute("delete from video", rusqlite::NO_PARAMS)?;
    conn.execute("delete from actress", rusqlite::NO_PARAMS)?;
    conn.execute("delete from video_actress", rusqlite::NO_PARAMS)?;

    // last unwrap is a pain to put into crate::error
    let path = SETTINGS.read().unwrap().get::<String>("path")?;
    for path in glob(&(path + "/*.json"))? {
        let data = fs::read_to_string(path?)?;
        let video: VideoConfig = serde_json::from_str(&data)?;
        conn.execute("INSERT INTO video (code, title, location, cover)
                     VALUES (?1, ?2, ?3, ?4)",
                     &[&video.code, &video.title, &video.location, &video.cover])?;
        let video_id = conn.last_insert_rowid();

        for actress in video.cast {
            conn.execute("insert into actress (name) select ?1
                         where not exists(select 1 from actress where name = ?1)",
                &[&actress])?;


            let actress_id = conn.last_insert_rowid();
            conn.execute("insert into video_actress (video_id, actress_id) values (?1, ?2)",
                &[video_id, actress_id])?;
        }
    }

    Ok(())
}

pub fn play_video(conn: rusqlite::Connection, id: i32) -> Result<(), Error> {
    let video = conn.query_row("select rowid, * from video where rowid = ?1",
                                  &[id],
                                  map_sql_to_video)?;
    Command::new("xdg-open")
        .arg(video.location)
        .output()?;
    Ok(())
}

pub fn search(conn: rusqlite::Connection, video_text: &str, actress_text: &str) -> Result<Vec<Video>, Error> {
    if video_text != "" && actress_text != "" {
        let mut stmt = conn.prepare(
            "select video.rowid, video.* from video
            join video_actress on video_actress.video_id = video.rowid
            where video_actress.actress_id in (select rowid from actress where name match ?2)
            and video match ?1")?;
        let video_iter = stmt.query_map(&[video_text, actress_text], map_sql_to_video).unwrap();
        Ok(video_iter.map(|video| video.unwrap()).collect())
    } else if video_text != "" && actress_text == "" {
        let mut stmt = conn.prepare("select rowid, * from video where video match ?1")?;
        let video_iter = stmt.query_map(&[video_text], map_sql_to_video)?;
        Ok(video_iter.map(|video| video.unwrap()).collect())
    } else if video_text == "" && actress_text != "" {
        let mut stmt = conn.prepare(
            "select video.rowid, video.* from video
            join video_actress on video_actress.video_id = video.rowid
            where video_actress.actress_id in (select rowid from actress where name match ?1)"
        )?;
        let video_iter = stmt.query_map(&[actress_text], map_sql_to_video)?;
        Ok(video_iter.map(|video| video.unwrap()).collect())
    } else {
        let mut stmt = conn.prepare("select rowid, * from video")?;
        let video_iter = stmt.query_map(rusqlite::NO_PARAMS, map_sql_to_video)?;
        Ok(video_iter.map(|video| video.unwrap()).collect())
    }
}
