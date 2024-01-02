-- Your SQL goes here
CREATE TABLE "songs" (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    "provider_id" INTEGER NOT NULL,
    "provider_song_id" TEXT NOT NULL,
    "created_at" INTEGER NOT NULL,
    "updated_at" INTEGER NOT NULL,
    FOREIGN KEY(provider_id) REFERENCES providers(id)
);