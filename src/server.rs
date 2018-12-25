extern crate iron;
extern crate mount;
extern crate serde_json;
extern crate staticfile;

use crate::backend;
use iron::prelude::*;
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use std::io::Read;

pub fn start_server() {
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("content/")));
    mount.mount("/api/scan_videos", scan_videos);
    mount.mount("/api/get_videos", get_videos);
    mount.mount("/api/play_video", play_video);
    Iron::new(mount).http("127.0.0.1:10010").unwrap();
}

fn scan_videos(_req: &mut Request) -> IronResult<Response> {
    backend::scan_videos();
    Ok(Response::with((iron::status::Ok, "".to_string())))
}

fn get_videos(_req: &mut Request) -> IronResult<Response> {
    let videos = backend::get_videos();
    Ok(Response::with((
        iron::status::Ok,
        serde_json::to_string(&videos).unwrap(),
    )))
}

fn play_video(req: &mut Request) -> IronResult<Response> {
    let mut body = String::new();
    req.body.read_to_string(&mut body).unwrap();
    backend::play_video(&body);
    Ok(Response::with((iron::status::Ok, "".to_string())))
}
