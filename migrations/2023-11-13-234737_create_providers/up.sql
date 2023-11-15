-- Your SQL goes here
CREATE TABLE "providers" (
	"id"	INTEGER,
	"name"	TEXT,
	"api"	TEXT,
	"username"	TEXT,
	"password"	TEXT,
	"ip"	TEXT,
	"port"	INTEGER,
	"connection_type"	INTEGER,
	PRIMARY KEY("id" AUTOINCREMENT)
);