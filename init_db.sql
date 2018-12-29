create virtual table video using fts5(
  title,
  code,
  location,
  cover
);

create virtual table actress using fts5(
  name
);

create table video_actress (
  id integer primary key,
  video_id integer,
  actress_id integer
);
