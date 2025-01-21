CREATE TABLE "levels" (
	"id"	INTEGER NOT NULL UNIQUE,
	"level_id"	INTEGER,
	"name"	TEXT NOT NULL,
	"verifier"	TEXT NOT NULL,
	"verification"	TEXT NOT NULL,
	"percent_to_qualify"	INTEGER NOT NULL,
	"song_name"	TEXT NOT NULL,
	"song_link"	TEXT,
	"difficulty"	INTEGER,
	PRIMARY KEY("id" AUTOINCREMENT)
)
