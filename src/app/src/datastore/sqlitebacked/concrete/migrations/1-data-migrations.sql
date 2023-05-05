CREATE TABLE IF NOT EXISTS data_migrations (
    id	INTEGER	PRIMARY KEY,
    name	TEXT	NOT NULL
);

CREATE UNIQUE INDEX data_migrations_unique_name ON data_migrations(name);
