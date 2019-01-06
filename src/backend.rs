extern crate glob;
extern crate regex;
extern crate rusqlite;
extern crate serde_derive;
extern crate serde_json;

use crate::config::SETTINGS;
use crate::error::Error;
use crate::models::*;
use glob::glob;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Deserialize)]
struct VideoConfig {
    code: String,
    title: String,
    location: String,
    cast: Vec<String>,
    genres: Vec<String>,
}

pub fn get_videos(conn: rusqlite::Connection) -> Result<Vec<Video>, Error> {
    let mut stmt = conn.prepare("SELECT rowid, * FROM video")?;
    let video_iter = stmt.query_map(rusqlite::NO_PARAMS, map_sql_to_video)?;
    Ok(video_iter.map(|video| video.unwrap()).collect())
}

pub fn scan_videos(mut conn: rusqlite::Connection) -> Result<(), Error> {
    let tx = conn.transaction()?;
    
    tx.execute("delete from video", rusqlite::NO_PARAMS)?;
    tx.execute("delete from actress", rusqlite::NO_PARAMS)?;
    tx.execute("delete from video_actress", rusqlite::NO_PARAMS)?;

    // last unwrap is a pain to put into crate::error
    let path = SETTINGS.read().unwrap().get::<String>("path")?.trim_end_matches("/").to_owned();
    for entry in glob(&(path + "/*.json"))? {
        let data = fs::read_to_string(entry?)?;

        let video: VideoConfig = serde_json::from_str(&data)?;
        tx.execute(
            "INSERT INTO video (code, title, location)
                     VALUES (?1, ?2, ?3)",
            &[&video.code, &video.title, &video.location],
        )?;
        let video_id = tx.last_insert_rowid();

        for actress in video.cast {
            tx.execute(
                "insert into actress (name) select ?1
                         where not exists(select 1 from actress where name = ?1)",
                &[&actress],
            )?;

            let actress_id = tx.query_row(
                "select rowid from actress where name = ?1",
                &[&actress],
                |row| row.get(0),
            )?;
            tx.execute(
                "insert into video_actress (video_id, actress_id) values (?1, ?2)",
                &[video_id, actress_id],
            )?;
        }

        for tag in video.genres {
            tx.execute("insert or ignore into tag (name) values (?1)", &[&tag])?;
            let tag_id = tx.query_row("select id from tag where name = ?1", &[&tag], |row| {
                row.get(0)
            })?;
            tx.execute(
                "insert into video_tag (video_id, tag_id) values (?1, ?2)",
                &[video_id, tag_id],
            )?;
        }
    }

    tx.commit()?;

    Ok(())
}

pub fn play_video(conn: rusqlite::Connection, id: i32) -> Result<(), Error> {
    let video = conn.query_row(
        "select rowid, * from video where rowid = ?1",
        &[id],
        map_sql_to_video,
    )?;
    let path = Path::new(&SETTINGS.read().unwrap().get::<String>("path")?)
        .join(&video.location)
        .as_os_str()
        .to_owned();
    Command::new("xdg-open").arg(&path).output()?;
    Ok(())
}

pub fn search(
    conn: rusqlite::Connection,
    video_text: &str,
    actress_text: &str,
    tags_text: &str,
) -> Result<Vec<Video>, Error> {
    let a_text = format!("%{}%", actress_text);
    let t_text = format!("%{}%", tags_text);

    let mut sql = String::from(
        "select distinct(video.rowid), video.* from video
	left join video_actress on video_actress.video_id = video.rowid
	left join actress on actress.rowid = video_actress.actress_id
	left join video_tag on video_tag.video_id = video.rowid
	left join tag on tag.id = video_tag.tag_id
	where actress.name like ?1
	and tag.name like ?2",
    );
    let mut values = vec![a_text, t_text];

    if video_text != "" {
        sql.push_str(" and video match ?3");
        values.push(String::from(video_text));
    }

    let mut stmt = conn.prepare(&sql)?;
    let video_iter = stmt.query_map(&values, map_sql_to_video)?;
    Ok(video_iter.map(|video| video.unwrap()).collect())
}
