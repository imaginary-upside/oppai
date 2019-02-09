create virtual table video using fts5(
  title,
  code,
  location,
  release_date
);

create virtual table actress using fts5(
  name,
  birthdate
);

create table video_actress (
  id integer primary key,
  video_id integer,
  actress_id integer
);

create table tag (
  id integer primary key,
  name text unique
);

create table video_tag (
  id integer primary key,
  video_id integer,
  tag_id integer
);

create index video_actress_video_id on video_actress (video_id);
create index video_actress_actress_id on video_actress (actress_id);
create index tag_name on tag (name);
create index video_tag_video_id on video_tag (video_id);
create index video_tag_tag_id on video_tag (tag_id);
