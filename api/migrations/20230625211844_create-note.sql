-- Add migration script here
CREATE TABLE notes
(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    content    TEXT not null,
    created_at TIMESTAMP not null DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    user_id    blob not null
);
