CREATE TABLE IF NOT EXISTS sources (
    id	INTEGER	PRIMARY KEY,
    name	TEXT	NOT NULL
);

CREATE UNIQUE INDEX sources_unique_name ON sources(name);
