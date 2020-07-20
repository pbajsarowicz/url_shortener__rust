CREATE TABLE links (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url VARCHAR NOT NULL,
    alias VARCHAR NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT 0
);

INSERT INTO links (url, alias, is_active) VALUES ("https://www.google.com", "google", 1);
INSERT INTO links (url, alias, is_active) VALUES ("https://www.wp.pl", "wp", 0);
