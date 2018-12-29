extern crate serde_derive;
extern crate serde_json;
extern crate rusqlite;

#[derive(Serialize)]
pub struct Video {
    pub id: i32,
    pub title: String,
    pub code: String,
    pub location: String,
    pub cover: String,
}

pub struct NewVideo {
    pub title: String,
    pub code: String,
    pub location: String,
    pub cover: String,
}

pub struct Actress {
    pub id: i32,
    pub name: String
}

pub struct NewActress {
    pub name: String
}

pub fn map_sql_to_video(row: &rusqlite::Row) -> Video {
    Video {
        id: row.get(0),
        title: row.get(1),
        code: row.get(2),
        location: row.get(3),
        cover: row.get(4)
    }
}

pub fn map_sql_to_actress(row: &rusqlite::Row) -> Actress {
    Actress {
        id: row.get(0),
        name: row.get(1),
    }
}
