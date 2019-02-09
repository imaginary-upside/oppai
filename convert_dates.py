import sqlite3

conn = sqlite3.connect("./database.sqlite")

c = conn.cursor()

c.execute("select rowid, birthdate from actress")
rows = c.fetchall()
for i in range(len(rows)):
    split = rows[i][1].split("/")
    if len(split) == 3:
        month, day, year = split
        if int(year) < 20:
            year = "20{}".format(year)
        else:
            year = "19{}".format(year)
        f = "{}-{}-{}".format(year, month, day)
        c.execute("update actress set birthdate=({}) where rowid=({})".format(f, rows[i][0]))
        print(f)

conn.commit()
conn.close()
