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
    pub release_date: String,
}

#[derive(Serialize)]
pub struct Actress {
    pub id: Option<i32>,
    pub name: String,
    pub birthdate: String,
}

pub struct Tag {
    pub id: Option<i32>,
    pub name: String,
}

pub fn map_sql_to_video(row: &rusqlite::Row) -> Video {
    let code: String = row.get(2);
    Video {
        id: Some(row.get(0)),
        title: row.get(1),
        code: code.to_owned(),
        location: row.get(3),
        cover: format!("{} Cover Thumb.jpg", code),
        release_date: row.get(4),
    }
}

pub fn map_sql_to_actress(row: &rusqlite::Row) -> Actress {
    Actress {
        id: Some(row.get(0)),
        name: row.get(1),
        birthdate: row.get(2),
    }
}

pub fn map_sql_to_tag(row: &rusqlite::Row) -> Tag {
    Tag {
        id: Some(row.get(0)),
        name: row.get(1),
    }
}
