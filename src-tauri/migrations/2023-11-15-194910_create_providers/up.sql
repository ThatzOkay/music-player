-- Your SQL goes here
CREATE TABLE "providers" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"api"	TEXT NOT NULL,
	"username"	TEXT NOT NULL,
	"password"	TEXT NOT NULL,
	"ip"	TEXT NOT NULL,
	"port"	INTEGER NOT NULL,
	"connection_type"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);