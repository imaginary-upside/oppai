table! {
    actresss (id) {
        id -> Nullable<Integer>,
        name -> Nullable<Text>,
    }
}

table! {
    video_actresss (id) {
        id -> Nullable<Integer>,
        video_id -> Integer,
        actress_id -> Integer,
    }
}

table! {
    videos (id) {
        id -> Nullable<Integer>,
        code -> Nullable<Text>,
        title -> Nullable<Text>,
        location -> Nullable<Text>,
        cover -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    actresss,
    video_actresss,
    videos,
);
