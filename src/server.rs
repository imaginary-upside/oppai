extern crate iron;
extern crate mount;
extern crate serde_json;
extern crate staticfile;
extern crate rusqlite;

use crate::backend;
use crate::error::Error;
use iron::prelude::*;
use mount::Mount;
use staticfile::Static;
use std::io::Read;
use std::path::Path;
use crate::config::SETTINGS;

#[derive(Deserialize)]
struct SearchData {
    video: String,
    actress: String
}

pub fn start_server() -> Result<(), Error> {
    let assets_path = SETTINGS.read().unwrap().get::<String>("path")?;
    
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("content/")));
    mount.mount("/assets/", Static::new(Path::new(&assets_path)));
    mount.mount("/api/scan_videos", scan_videos);
    mount.mount("/api/get_videos", get_videos);
    mount.mount("/api/play_video", play_video);
    mount.mount("/api/search", search);
    Iron::new(mount).http("127.0.0.1:10010").expect("could not attach to 127.0.0.1:10010");

    Ok(())
}

fn scan_videos(_req: &mut Request) -> IronResult<Response> {
    let conn = rusqlite::Connection::open("database.sqlite").unwrap();
    backend::scan_videos(conn).unwrap();
    Ok(Response::with((iron::status::Ok, "".to_string())))
}

fn get_videos(_req: &mut Request) -> IronResult<Response> {
    let conn = rusqlite::Connection::open("database.sqlite").unwrap();
    let videos = backend::get_videos(conn);
    Ok(Response::with((
        iron::status::Ok,
        serde_json::to_string(&videos.unwrap()).unwrap(),
    )))
}

fn play_video(req: &mut Request) -> IronResult<Response> {
    let conn = rusqlite::Connection::open("database.sqlite").unwrap();
    let mut body = String::new();
    req.body.read_to_string(&mut body).unwrap();
    backend::play_video(conn, body.parse::<i32>().unwrap()).unwrap();
    Ok(Response::with(iron::status::Ok))
}

fn search(req: &mut Request) -> IronResult<Response> {
    let conn = rusqlite::Connection::open("database.sqlite").unwrap();
    let mut body = String::new();
    req.body.read_to_string(&mut body).unwrap();
    let search_data: SearchData = serde_json::from_str(&body).unwrap();
    let videos = backend::search(conn, &search_data.video, &search_data.actress).unwrap();
    Ok(Response::with((iron::status::Ok, serde_json::to_string(&videos).unwrap())))
}
