extern crate rusqlite;
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize)]
pub struct Video {
    pub id: Option<i32>,
    pub title: String,
    pub code: String,
    pub location: String,
    pub cover: String,
}

pub struct Actress {
    pub id: Option<i32>,
    pub name: String,
}

pub struct Tag {
    pub id: Option<i32>,
    pub name: String,
}

pub fn map_sql_to_video(row: &rusqlite::Row) -> Video {
    Video {
        id: Some(row.get(0)),
        title: row.get(1),
        code: row.get(2),
        location: row.get(3),
        cover: row.get(4),
    }
}

pub fn map_sql_to_actress(row: &rusqlite::Row) -> Actress {
    Actress {
        id: Some(row.get(0)),
        name: row.get(1),
    }
}

pub fn map_sql_to_tag(row: &rusqlite::Row) -> Tag {
    Tag {
        id: Some(row.get(0)),
        name: row.get(1),
    }
}
