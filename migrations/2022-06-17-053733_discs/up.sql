CREATE TABLE discs (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    title_id        TEXT NOT NULL,
    path            TEXT NOT NULL,
    media           TEXT CHECK ( media IN ('CD', 'DVD') ),
    format          TEXT CHECK ( format IN ('ISO') ),
    platform        TEXT CHECK ( platform IN ('PS2') ),
    size            INTEGER NOT NULL,
    title           TEXT,
    description     TEXT,
    developer       TEXT,
    genre           TEXT,
    video_mode      TEXT CHECK ( video_mode ISNULL OR video_mode IN ('multi', 'PAL', 'NTSC') ),
    release         DATE,
    players         INTEGER CHECK ( players ISNULL OR ( players >= 1 AND players <= 8 ) ),
    rating_stars    INTEGER,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIME,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIME
);
